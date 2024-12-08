#[cfg(feature = "UnityEngine+ParticleSystemBakeTextureOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleSystemBakeTextureOptions {
    BakePosition = 2i32,
    BakeRotationAndScale = 1i32,
    Default = 4i32,
    IncludeParticleIndices = 16i32,
    PerParticle = 8i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemBakeTextureOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemBakeTextureOptions =>
    "UnityEngine"."ParticleSystemBakeTextureOptions"
);
