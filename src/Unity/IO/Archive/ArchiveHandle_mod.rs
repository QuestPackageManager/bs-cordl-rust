#[cfg(feature = "Unity+IO+Archive+ArchiveHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ArchiveHandle {
    pub Handle: u64,
}
#[cfg(feature = "Unity+IO+Archive+ArchiveHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::IO::Archive::ArchiveHandle =>
    "Unity.IO.Archive"."ArchiveHandle"
);
#[cfg(feature = "Unity+IO+Archive+ArchiveHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::IO::Archive::ArchiveHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveHandle")]
impl crate::Unity::IO::Archive::ArchiveHandle {}
