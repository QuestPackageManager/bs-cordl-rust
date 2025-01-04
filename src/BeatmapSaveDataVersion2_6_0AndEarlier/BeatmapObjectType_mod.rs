#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapObjectType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BeatmapObjectType {
    #[default]
    Note = 0i32,
    Obstacle = 2i32,
    Slider = 3i32,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapObjectType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapObjectType =>
    "BeatmapSaveDataVersion2_6_0AndEarlier"."BeatmapObjectType"
);
