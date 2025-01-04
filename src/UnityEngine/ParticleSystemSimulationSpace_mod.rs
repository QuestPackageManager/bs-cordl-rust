#[cfg(feature = "UnityEngine+ParticleSystemSimulationSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleSystemSimulationSpace {
    #[default]
    Custom = 2i32,
    Local = 0i32,
    World = 1i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemSimulationSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemSimulationSpace =>
    "UnityEngine"."ParticleSystemSimulationSpace"
);
