#[cfg(feature = "UnityEngine+ParticleSystemBakeMeshOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleSystemBakeMeshOptions {
    #[default]
    BakePosition = 2i32,
    BakeRotationAndScale = 1i32,
    Default = 0i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemBakeMeshOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemBakeMeshOptions =>
    "UnityEngine"."ParticleSystemBakeMeshOptions"
);
