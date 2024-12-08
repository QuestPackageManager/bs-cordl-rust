#[cfg(feature = "UnityEngine+CollisionDetectionMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionDetectionMode {
    Continuous = 1i32,
    ContinuousDynamic = 2i32,
    ContinuousSpeculative = 3i32,
    Discrete = 0i32,
}
#[cfg(feature = "UnityEngine+CollisionDetectionMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CollisionDetectionMode =>
    "UnityEngine"."CollisionDetectionMode"
);
