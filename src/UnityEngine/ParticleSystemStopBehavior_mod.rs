#[cfg(feature = "UnityEngine+ParticleSystemStopBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleSystemStopBehavior {
    #[default]
    StopEmitting = 1i32,
    StopEmittingAndClear = 0i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemStopBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemStopBehavior =>
    "UnityEngine"."ParticleSystemStopBehavior"
);
