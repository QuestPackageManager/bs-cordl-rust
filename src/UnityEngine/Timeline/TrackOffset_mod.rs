#[cfg(feature = "UnityEngine+Timeline+TrackOffset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackOffset {
    ApplySceneOffsets = 1i32,
    ApplyTransformOffsets = 0i32,
    Auto = 2i32,
}
#[cfg(feature = "UnityEngine+Timeline+TrackOffset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TrackOffset =>
    "UnityEngine.Timeline"."TrackOffset"
);
