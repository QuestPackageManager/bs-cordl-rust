#[cfg(feature = "UnityEngine+Timeline+MatchTargetFields")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchTargetFields {
    PositionX = 1i32,
    PositionY = 2i32,
    PositionZ = 4i32,
    RotationX = 8i32,
    RotationY = 16i32,
    RotationZ = 32i32,
}
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFields")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::MatchTargetFields =>
    "UnityEngine.Timeline"."MatchTargetFields"
);
