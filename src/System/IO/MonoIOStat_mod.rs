#[cfg(feature = "System+IO+MonoIOStat")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MonoIOStat {
    pub fileAttributes: crate::System::IO::FileAttributes,
    pub Length: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
}
#[cfg(feature = "System+IO+MonoIOStat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::MonoIOStat => "System.IO"
    ."MonoIOStat"
);
#[cfg(feature = "System+IO+MonoIOStat")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::IO::MonoIOStat {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+IO+MonoIOStat")]
impl crate::System::IO::MonoIOStat {}
