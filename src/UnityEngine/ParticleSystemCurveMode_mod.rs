#[cfg(feature = "UnityEngine+ParticleSystemCurveMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleSystemCurveMode {
    Constant = 0i32,
    Curve = 1i32,
    TwoConstants = 3i32,
    TwoCurves = 2i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemCurveMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemCurveMode =>
    "UnityEngine"."ParticleSystemCurveMode"
);
