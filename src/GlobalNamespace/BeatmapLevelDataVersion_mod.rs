#[cfg(feature = "BeatmapLevelDataVersion")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BeatmapLevelDataVersion {
    #[default]
    NoEnvironmentKeywords = 1i32,
    Original = 0i32,
}
#[cfg(feature = "BeatmapLevelDataVersion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelDataVersion => ""
    ."BeatmapLevelDataVersion"
);
