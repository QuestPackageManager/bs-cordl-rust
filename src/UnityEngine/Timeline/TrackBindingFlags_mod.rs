#[cfg(feature = "UnityEngine+Timeline+TrackBindingFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackBindingFlags {
    #[default]
    All = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+Timeline+TrackBindingFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TrackBindingFlags =>
    "UnityEngine.Timeline"."TrackBindingFlags"
);
