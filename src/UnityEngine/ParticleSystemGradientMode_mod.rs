#[cfg(feature = "UnityEngine+ParticleSystemGradientMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleSystemGradientMode {
    #[default]
    Color = 0i32,
    Gradient = 1i32,
    RandomColor = 4i32,
    TwoColors = 2i32,
    TwoGradients = 3i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemGradientMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemGradientMode =>
    "UnityEngine"."ParticleSystemGradientMode"
);
