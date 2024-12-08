#[cfg(feature = "BeatmapEventTransitionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeatmapEventTransitionType {
    Extend = 2i32,
    Instant = 0i32,
    Interpolate = 1i32,
}
#[cfg(feature = "BeatmapEventTransitionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapEventTransitionType =>
    ""."BeatmapEventTransitionType"
);
