#[cfg(feature = "UnityEngine+XR+MeshGenerationStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshGenerationStatus {
    #[default]
    Canceled = 3i32,
    GenerationAlreadyInProgress = 2i32,
    InvalidMeshId = 1i32,
    Success = 0i32,
    UnknownError = 4i32,
}
#[cfg(feature = "UnityEngine+XR+MeshGenerationStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::MeshGenerationStatus =>
    "UnityEngine.XR"."MeshGenerationStatus"
);
