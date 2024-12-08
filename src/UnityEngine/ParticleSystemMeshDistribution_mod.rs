#[cfg(feature = "UnityEngine+ParticleSystemMeshDistribution")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleSystemMeshDistribution {
    NonUniformRandom = 1i32,
    UniformRandom = 0i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemMeshDistribution")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemMeshDistribution =>
    "UnityEngine"."ParticleSystemMeshDistribution"
);
