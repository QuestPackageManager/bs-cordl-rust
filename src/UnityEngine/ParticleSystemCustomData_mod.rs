#[cfg(feature = "UnityEngine+ParticleSystemCustomData")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleSystemCustomData {
    Custom1 = 0i32,
    Custom2 = 1i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemCustomData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemCustomData =>
    "UnityEngine"."ParticleSystemCustomData"
);