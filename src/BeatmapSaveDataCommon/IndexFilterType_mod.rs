#[cfg(feature = "BeatmapSaveDataCommon+IndexFilterType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IndexFilterType {
    #[default]
    Division = 1i32,
    StepAndOffset = 2i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+IndexFilterType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataCommon::IndexFilterType =>
    "BeatmapSaveDataCommon"."IndexFilterType"
);
