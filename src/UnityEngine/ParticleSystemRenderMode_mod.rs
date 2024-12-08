#[cfg(feature = "UnityEngine+ParticleSystemRenderMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleSystemRenderMode {
    Billboard = 0i32,
    HorizontalBillboard = 2i32,
    Mesh = 4i32,
    None = 5i32,
    Stretch = 1i32,
    VerticalBillboard = 3i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemRenderMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemRenderMode =>
    "UnityEngine"."ParticleSystemRenderMode"
);
