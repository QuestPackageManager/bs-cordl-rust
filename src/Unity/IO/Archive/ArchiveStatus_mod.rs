#[cfg(feature = "Unity+IO+Archive+ArchiveStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArchiveStatus {
    #[default]
    Complete = 1i32,
    Failed = 2i32,
    InProgress = 0i32,
}
#[cfg(feature = "Unity+IO+Archive+ArchiveStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::IO::Archive::ArchiveStatus =>
    "Unity.IO.Archive"."ArchiveStatus"
);
