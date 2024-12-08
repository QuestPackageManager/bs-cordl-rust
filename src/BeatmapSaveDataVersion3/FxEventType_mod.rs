#[cfg(feature = "BeatmapSaveDataVersion3+FxEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FxEventType {
    Bool = 2i32,
    Float = 1i32,
    Int = 0i32,
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::FxEventType =>
    "BeatmapSaveDataVersion3"."FxEventType"
);
