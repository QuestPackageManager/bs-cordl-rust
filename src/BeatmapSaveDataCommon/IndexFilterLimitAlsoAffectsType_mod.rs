#[cfg(feature = "BeatmapSaveDataCommon+IndexFilterLimitAlsoAffectsType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IndexFilterLimitAlsoAffectsType {
    #[default]
    Distribution = 2i32,
    Duration = 1i32,
    None = 0i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+IndexFilterLimitAlsoAffectsType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataCommon::IndexFilterLimitAlsoAffectsType => "BeatmapSaveDataCommon"
    ."IndexFilterLimitAlsoAffectsType"
);
