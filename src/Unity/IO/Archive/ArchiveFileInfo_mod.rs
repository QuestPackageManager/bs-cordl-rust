#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ArchiveFileInfo {
    pub Filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub FileSize: u64,
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::IO::Archive::ArchiveFileInfo =>
    "Unity.IO.Archive"."ArchiveFileInfo"
);
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::IO::Archive::ArchiveFileInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
impl crate::Unity::IO::Archive::ArchiveFileInfo {}
