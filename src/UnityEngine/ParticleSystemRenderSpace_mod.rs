#[cfg(feature = "UnityEngine+ParticleSystemRenderSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleSystemRenderSpace {
    #[default]
    Facing = 3i32,
    Local = 2i32,
    Velocity = 4i32,
    View = 0i32,
    World = 1i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemRenderSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemRenderSpace =>
    "UnityEngine"."ParticleSystemRenderSpace"
);
