#[cfg(feature = "UnityEngine+JointProjectionMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JointProjectionMode {
    #[default]
    None = 0i32,
    PositionAndRotation = 1i32,
    PositionOnly = 2i32,
}
#[cfg(feature = "UnityEngine+JointProjectionMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::JointProjectionMode =>
    "UnityEngine"."JointProjectionMode"
);
