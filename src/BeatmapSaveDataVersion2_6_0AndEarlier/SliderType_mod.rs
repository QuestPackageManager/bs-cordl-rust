#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderType {
    Burst = 1i32,
    Normal = 0i32,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion2_6_0AndEarlier::SliderType =>
    "BeatmapSaveDataVersion2_6_0AndEarlier"."SliderType"
);
