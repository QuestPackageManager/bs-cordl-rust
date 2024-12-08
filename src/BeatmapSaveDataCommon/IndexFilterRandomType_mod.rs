#[cfg(feature = "BeatmapSaveDataCommon+IndexFilterRandomType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFilterRandomType {
    KeepOrder = 1i32,
    NoRandom = 0i32,
    RandomElements = 2i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+IndexFilterRandomType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataCommon::IndexFilterRandomType =>
    "BeatmapSaveDataCommon"."IndexFilterRandomType"
);
