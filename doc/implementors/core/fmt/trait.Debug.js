(function() {var implementors = {};
implementors["hal"] = [{"text":"impl Debug for MemoryType","synthetic":false,"types":[]},{"text":"impl Debug for MemoryMapEntry","synthetic":false,"types":[]},{"text":"impl Debug for LoadedImage","synthetic":false,"types":[]},{"text":"impl Debug for Segment","synthetic":false,"types":[]},{"text":"impl Debug for VideoModeInfo","synthetic":false,"types":[]},{"text":"impl Debug for PixelFormat","synthetic":false,"types":[]},{"text":"impl&lt;S:&nbsp;Debug&gt; Debug for Frame&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: FrameSize,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S:&nbsp;Debug + FrameSize&gt; Debug for Page&lt;S&gt;","synthetic":false,"types":[]},{"text":"impl Debug for Flags","synthetic":false,"types":[]},{"text":"impl Debug for PagingError","synthetic":false,"types":[]},{"text":"impl Debug for PhysicalAddress","synthetic":false,"types":[]},{"text":"impl Debug for VirtualAddress","synthetic":false,"types":[]},{"text":"impl Debug for Size4KiB","synthetic":false,"types":[]},{"text":"impl Debug for Size2MiB","synthetic":false,"types":[]},{"text":"impl Debug for Size1GiB","synthetic":false,"types":[]}];
implementors["kernel"] = [{"text":"impl Debug for HoleInfo","synthetic":false,"types":[]},{"text":"impl Debug for Allocation","synthetic":false,"types":[]},{"text":"impl Debug for Stack","synthetic":false,"types":[]},{"text":"impl Debug for State","synthetic":false,"types":[]},{"text":"impl Debug for TaskBlock","synthetic":false,"types":[]},{"text":"impl Debug for TaskState","synthetic":false,"types":[]},{"text":"impl Debug for TaskCreationError","synthetic":false,"types":[]},{"text":"impl Debug for KernelObjectId","synthetic":false,"types":[]}];
implementors["libpebble"] = [{"text":"impl Debug for Capability","synthetic":false,"types":[]},{"text":"impl Debug for GetFramebufferError","synthetic":false,"types":[]},{"text":"impl Debug for PixelFormat","synthetic":false,"types":[]},{"text":"impl Debug for FramebufferInfo","synthetic":false,"types":[]},{"text":"impl Debug for PciDeviceInfo","synthetic":false,"types":[]},{"text":"impl Debug for Bar","synthetic":false,"types":[]},{"text":"impl Debug for PciGetInfoError","synthetic":false,"types":[]},{"text":"impl Debug for EarlyLogError","synthetic":false,"types":[]},{"text":"impl Debug for CreateMemoryObjectError","synthetic":false,"types":[]},{"text":"impl Debug for MapMemoryObjectError","synthetic":false,"types":[]},{"text":"impl Debug for SendMessageError","synthetic":false,"types":[]},{"text":"impl Debug for GetMessageError","synthetic":false,"types":[]},{"text":"impl Debug for RegisterServiceError","synthetic":false,"types":[]},{"text":"impl Debug for SubscribeToServiceError","synthetic":false,"types":[]},{"text":"impl Debug for Handle","synthetic":false,"types":[]}];
implementors["log"] = [{"text":"impl Debug for Level","synthetic":false,"types":[]},{"text":"impl Debug for LevelFilter","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Debug for Record&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Debug for RecordBuilder&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Debug for Metadata&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Debug for MetadataBuilder&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Debug for SetLoggerError","synthetic":false,"types":[]},{"text":"impl Debug for ParseLevelError","synthetic":false,"types":[]}];
implementors["num_traits"] = [{"text":"impl Debug for FloatErrorKind","synthetic":false,"types":[]},{"text":"impl Debug for ParseFloatError","synthetic":false,"types":[]}];
implementors["pci_types"] = [{"text":"impl Debug for DeviceType","synthetic":false,"types":[]},{"text":"impl Debug for UsbType","synthetic":false,"types":[]},{"text":"impl Debug for PciAddress","synthetic":false,"types":[]},{"text":"impl Debug for Bar","synthetic":false,"types":[]}];
implementors["pebble_util"] = [{"text":"impl&lt;T:&nbsp;Binary + PrimInt&gt; Debug for BinaryPrettyPrint&lt;T&gt;","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Debug for TokenStream","synthetic":false,"types":[]},{"text":"impl Debug for LexError","synthetic":false,"types":[]},{"text":"impl Debug for Span","synthetic":false,"types":[]},{"text":"impl Debug for TokenTree","synthetic":false,"types":[]},{"text":"impl Debug for Delimiter","synthetic":false,"types":[]},{"text":"impl Debug for Group","synthetic":false,"types":[]},{"text":"impl Debug for Spacing","synthetic":false,"types":[]},{"text":"impl Debug for Punct","synthetic":false,"types":[]},{"text":"impl Debug for Ident","synthetic":false,"types":[]},{"text":"impl Debug for Literal","synthetic":false,"types":[]},{"text":"impl Debug for IntoIter","synthetic":false,"types":[]}];
implementors["ptah"] = [{"text":"impl Debug for Error","synthetic":false,"types":[]},{"text":"impl Debug for Error","synthetic":false,"types":[]}];
implementors["spin"] = [{"text":"impl&lt;'a, T:&nbsp;Debug + ?Sized + 'a&gt; Debug for MutexGuard&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized + Debug&gt; Debug for Mutex&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Debug + 'a + ?Sized&gt; Debug for RwLockReadGuard&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Debug + 'a + ?Sized&gt; Debug for RwLockWriteGuard&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Debug + 'a + ?Sized&gt; Debug for RwLockUpgradeableGuard&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized + Debug&gt; Debug for RwLock&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Debug&gt; Debug for Once&lt;T&gt;","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;'a&gt; Debug for ParseBuffer&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Debug for Error","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()