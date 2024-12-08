#[cfg(feature = "BeatmapSaveDataVersion3+TransitionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    Extend = 2i32,
    Instant = 0i32,
    Interpolate = 1i32,
}
#[cfg(feature = "BeatmapSaveDataVersion3+TransitionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::TransitionType =>
    "BeatmapSaveDataVersion3"."TransitionType"
);
