#[cfg(feature = "UnityEngine+ParticleSystemScalingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleSystemScalingMode {
    Hierarchy = 0i32,
    Local = 1i32,
    Shape = 2i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemScalingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemScalingMode =>
    "UnityEngine"."ParticleSystemScalingMode"
);
