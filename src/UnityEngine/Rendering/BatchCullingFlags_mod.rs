#[cfg(feature = "UnityEngine+Rendering+BatchCullingFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchCullingFlags {
    CullLightmappedShadowCasters = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+Rendering+BatchCullingFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchCullingFlags =>
    "UnityEngine.Rendering"."BatchCullingFlags"
);
