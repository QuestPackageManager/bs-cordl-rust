#[cfg(feature = "UnityEngine+Rendering+MeshUpdateFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshUpdateFlags {
    #[default]
    Default = 0i32,
    DontNotifyMeshUsers = 4i32,
    DontRecalculateBounds = 8i32,
    DontResetBoneBounds = 2i32,
    DontValidateIndices = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+MeshUpdateFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::MeshUpdateFlags =>
    "UnityEngine.Rendering"."MeshUpdateFlags"
);
