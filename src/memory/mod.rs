/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

mod area_frame_allocator;
mod paging;

pub use self::paging::test_paging;

pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::remap_kernel;
use self::paging::PhysicalAddress;

pub const PAGE_SIZE : usize = 4096;

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct Frame
{
    number : usize
}

struct FrameIter
{
    start : Frame,
    end   : Frame
}

impl Iterator for FrameIter
{
    type Item = Frame;

    fn next(&mut self) -> Option<Frame>
    {
        if self.start <= self.end
        {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        }
        else
        {
            None
        }
    }
}

impl Frame
{
    fn get_containing_frame(address : usize) -> Frame
    {
        Frame { number : address / PAGE_SIZE }
    }

    fn get_start_address(&self) -> PhysicalAddress
    {
        self.number * PAGE_SIZE
    }

    fn clone(&self) -> Frame
    {
        Frame { number : self.number }
    }

    fn range_inclusive(start : Frame, end : Frame) -> FrameIter
    {
        FrameIter
        {
            start : start,
            end   : end
        }
    }
}

pub trait FrameAllocator
{
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame : Frame);
}
