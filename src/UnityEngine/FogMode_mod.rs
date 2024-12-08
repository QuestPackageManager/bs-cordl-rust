#[cfg(feature = "UnityEngine+FogMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FogMode {
    Exponential = 2i32,
    ExponentialSquared = 3i32,
    Linear = 1i32,
}
#[cfg(feature = "UnityEngine+FogMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FogMode => "UnityEngine"."FogMode"
);
