pub mod entry;
pub mod frame;
pub mod frame_allocator;
pub mod mapper;
pub mod page;
pub mod table;

pub use self::{frame::Frame, frame_allocator::FrameAllocator, mapper::Mapper, page::Page};
pub use core::ops::{Deref, DerefMut};

pub const FRAME_SIZE: usize = 0x1000;
pub const PAGE_SIZE: usize = 0x1000;

use self::{
    entry::EntryFlags,
    table::{
        IdentityMapping,
        Level4,
        RecursiveMapping,
        Table,
        TableMapping,
        KERNEL_P4_NON_RECURSIVE,
    },
};
use super::{PhysicalAddress, VirtualAddress};
use crate::hw::{
    registers::{read_control_reg, write_control_reg},
    tlb,
};
use core::marker::PhantomData;

/// Represents a set of page tables that are not currently mapped.
pub struct InactivePageTable<M>
where
    M: TableMapping,
{
    pub p4_frame: Frame,
    _mapping: PhantomData<M>,
}

impl<M> InactivePageTable<M>
where
    M: TableMapping,
{
    /// Create a new set of page-tables. `frame` must be an allocated, **zeroed** `Frame` of
    /// physical memory. We don't zero the memory here because to do that we need to map it into
    /// the active set of page tables, which aren't available when we first create an
    /// `InactivePageTable` in the bootloader.
    ///
    /// If you're absolutely sure the given `Frame` contains a valid P4 page table, you can also
    /// use this to construct a page table. This is very unsafe as you need to make sure you're
    /// correctly managing the backing physical memory, and that the pre-installed table has the
    /// correct mapping (if `M` isn't `NoMapping`).
    ///
    /// Unsafe because we assume `frame` is a free, zeroed (or contains a valid P4 page table)
    /// frame of physical memory.
    pub unsafe fn new(frame: Frame) -> InactivePageTable<M> {
        InactivePageTable { p4_frame: frame, _mapping: PhantomData }
    }
}

impl InactivePageTable<IdentityMapping> {
    /// Create a new set of page-tables that should be accessed using identity mapping, but which
    /// also have the correct entries for recursive mapping. This means they can be created in a
    /// context with an identity mapping, but when switched to, can correctly form an
    /// `ActivePageTable<RecursiveMapping>`.
    ///
    /// Unsafe because we assume that `frame` is a free, zeroed frame of physical memory.
    pub unsafe fn new_with_recursive_mapping(
        frame: Frame,
        recursive_entry: u16,
    ) -> InactivePageTable<IdentityMapping> {
        let table = InactivePageTable { p4_frame: frame, _mapping: PhantomData };

        let p4: &mut Table<Level4, IdentityMapping> =
            &mut *(VirtualAddress::new_unchecked(usize::from(frame.start_address())).mut_ptr());
        p4[recursive_entry].set(frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);

        table
    }

    pub fn mapper<'a>(&'a mut self) -> Mapper<'a, IdentityMapping> {
        unsafe { Mapper::<IdentityMapping>::new(self.p4_frame.start_address()) }
    }

    /// Switch to this set of page tables. This returns a tuple containing the new
    /// `ActivePageTable` (that this has become), and the previously-active set of tables as an
    /// `InactivePageTable`.
    ///
    /// Unsafe because you are required to specify the correct `TableMapping` for the currently
    /// installed set of page tables (the one that is returned as an `InactivePageTable<A>`), as
    /// this can't be type-checked.
    ///
    /// # Generic parameters
    /// The generic parameter `N` represents the `TableMapping` that the currently installed set of
    /// page tables have. This is used to construct the returned `InactivePageTable<N>`.
    pub unsafe fn switch_to<N>(self) -> (ActivePageTable<IdentityMapping>, InactivePageTable<N>)
    where
        N: TableMapping,
    {
        let old_table_address = PhysicalAddress::new(read_control_reg!(cr3) as usize).unwrap();

        // NOTE: We don't need to flush the TLB here because it's cleared when CR3 changes
        write_control_reg!(cr3, usize::from(self.p4_frame.start_address()) as u64);

        (
            ActivePageTable::<IdentityMapping>::new(self.p4_frame.start_address()),
            InactivePageTable::<N>::new(Frame::contains(old_table_address)),
        )
    }
}

impl InactivePageTable<RecursiveMapping> {
    /// Switch to this set of page tables. This returns a tuple containing the new
    /// `ActivePageTable` (that this has become), and the previously-active set of tables as an
    /// `InactivePageTable`.
    ///
    /// Unsafe because you are required to specify the correct `TableMapping` for the currently
    /// installed set of page tables (the one that is returned as an `InactivePageTable<A>`), as
    /// this can't be type-checked.
    ///
    /// # Generic parameters
    /// The generic parameter `N` represents the `TableMapping` that the currently installed set of
    /// page tables have. This is used to construct the returned `InactivePageTable<N>`.
    pub unsafe fn switch_to<N>(self) -> (ActivePageTable<RecursiveMapping>, InactivePageTable<N>)
    where
        N: TableMapping,
    {
        let old_table_address = PhysicalAddress::new(read_control_reg!(cr3) as usize).unwrap();

        // NOTE: We don't need to flush the TLB here because it's cleared when CR3 changes
        write_control_reg!(cr3, usize::from(self.p4_frame.start_address()) as u64);

        (
            ActivePageTable::<RecursiveMapping>::new(),
            InactivePageTable::<N>::new(Frame::contains(old_table_address)),
        )
    }
}

/// Represents the set of page tables that are currently being used. The recursive mapping will
/// point to the address of these tables, and so it's safe to create a `Mapper` for an
/// `ActivePageTable`.
pub struct ActivePageTable<M>
where
    M: 'static + TableMapping,
{
    mapper: Mapper<'static, M>,
}

impl<M> Deref for ActivePageTable<M>
where
    M: 'static + TableMapping,
{
    type Target = Mapper<'static, M>;

    fn deref(&self) -> &Self::Target {
        &self.mapper
    }
}

impl<M> DerefMut for ActivePageTable<M>
where
    M: 'static + TableMapping,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mapper
    }
}

impl ActivePageTable<IdentityMapping> {
    /// Create an `ActivePageTable` to represent an active set of page tables that should be
    /// accessed directly using their physical addresses. This only works in an environment with an
    /// identity-mapped virtual address space (such as in the UEFI bootloader), and should be used
    /// before we have a set of page tables that are recursively mapped.
    pub unsafe fn new(p4_address: PhysicalAddress) -> ActivePageTable<IdentityMapping> {
        ActivePageTable { mapper: Mapper::<IdentityMapping>::new(p4_address) }
    }
}

impl ActivePageTable<RecursiveMapping> {
    /// Create an `ActivePageTable` to represent the currently-installed set of page tables. This
    /// is unsafe because it assumes a valid set of page tables exist and are pointed to by `CR3`,
    /// and that they are correctly recursively mapped.
    pub unsafe fn new() -> ActivePageTable<RecursiveMapping> {
        ActivePageTable { mapper: Mapper::<RecursiveMapping>::new() }
    }

    /// Alter the mappings of a `InactivePageTable` by temporarily replacing the recursive entry
    /// address of the active tables with the physical address of the inactive table's P4.
    ///
    /// The `InactivePageTable` must also be recursively-mapped for this to work.
    ///
    /// This calls the closure with a `Mapper` that targets the current set of active tables, but
    /// will actually modify the given `InactivePageTable`'s mappings. Because the inactive table
    /// isn't really mapped, you can't modify the *contents* of the mappings. To modify the
    /// physical memory, you will either need to switch to the `InactivePageTable`, or map the
    /// memory you want to modify into the `ActivePageTable` temporarily.
    pub fn with<A, F, R>(
        &mut self,
        table: &mut InactivePageTable<RecursiveMapping>,
        allocator: &A,
        f: F,
    ) -> R
    where
        A: FrameAllocator,
        F: FnOnce(&mut Mapper<RecursiveMapping>, &A) -> R,
    {
        use super::kernel_map::RECURSIVE_ENTRY;

        /*
         * Backup the kernel P4's physical address.
         */
        let kernel_p4_frame =
            Frame::contains(PhysicalAddress::new(read_control_reg!(cr3) as usize).unwrap());

        /*
         * Overwrite recursive mapping, so it points to the inactive page table. Flush the TLB
         * because it might contain stale mappings to the kernel's recursive entries.
         */
        self.p4[RECURSIVE_ENTRY].set(table.p4_frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        tlb::flush();

        /*
         * Execute the closure with the new recursive mapping.
         *
         * NOTE: We make sure to only give it `self` as a `&mut Mapper<RecursiveMapping>`,
         * because if we gave it a `ActivePageTable`, it could call `with` again, which
         * would not work.
         */
        let result = f(self, allocator);

        /*
         * Restore the kernel's recursive mapping and flush the TLB again.
         *
         * NOTE: we make sure not to use `self.p4` here because that relies on the recursive
         * mapping.
         */
        let kernel_p4 = unsafe { &mut *KERNEL_P4_NON_RECURSIVE };
        kernel_p4[RECURSIVE_ENTRY].set(kernel_p4_frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        tlb::flush();

        result
    }
}