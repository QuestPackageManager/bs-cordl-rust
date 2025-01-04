#[cfg(feature = "UnityEngine+ParticleSystemCustomData")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleSystemCustomData {
    #[default]
    Custom1 = 0i32,
    Custom2 = 1i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemCustomData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemCustomData =>
    "UnityEngine"."ParticleSystemCustomData"
);
