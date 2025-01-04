#[cfg(feature = "BeatmapSaveDataCommon+ExecutionTime")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExecutionTime {
    #[default]
    Early = 0i32,
    Late = 1i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+ExecutionTime")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataCommon::ExecutionTime =>
    "BeatmapSaveDataCommon"."ExecutionTime"
);
