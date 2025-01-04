#[cfg(feature = "BeatmapSaveDataCommon+DistributionParamType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DistributionParamType {
    #[default]
    Step = 2i32,
    Wave = 1i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+DistributionParamType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataCommon::DistributionParamType =>
    "BeatmapSaveDataCommon"."DistributionParamType"
);
