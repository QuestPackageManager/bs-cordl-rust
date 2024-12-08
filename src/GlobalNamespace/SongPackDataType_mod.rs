#[cfg(feature = "SongPackDataType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SongPackDataType {
    All = 2i32,
    MultipleBeatmapLevelsPack = 1i32,
    SingleBeatmapLevelPack = 0i32,
}
#[cfg(feature = "SongPackDataType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongPackDataType => ""
    ."SongPackDataType"
);
