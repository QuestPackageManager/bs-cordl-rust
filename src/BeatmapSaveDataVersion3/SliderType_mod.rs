#[cfg(feature = "BeatmapSaveDataVersion3+SliderType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderType {
    Burst = 1i32,
    Normal = 0i32,
}
#[cfg(feature = "BeatmapSaveDataVersion3+SliderType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::SliderType =>
    "BeatmapSaveDataVersion3"."SliderType"
);
