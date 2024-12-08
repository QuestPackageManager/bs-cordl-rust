#[cfg(feature = "UnityEngine+Rendering+CullingOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullingOptions {
    DisablePerObjectCulling = 32i32,
    ForceEvenIfCameraIsNotActive = 1i32,
    NeedsLighting = 4i32,
    NeedsReflectionProbes = 8i32,
    None = 0i32,
    OcclusionCull = 2i32,
    ShadowCasters = 64i32,
    Stereo = 16i32,
}
#[cfg(feature = "UnityEngine+Rendering+CullingOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::CullingOptions =>
    "UnityEngine.Rendering"."CullingOptions"
);
