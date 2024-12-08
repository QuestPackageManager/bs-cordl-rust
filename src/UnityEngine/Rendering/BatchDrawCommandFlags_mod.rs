#[cfg(feature = "UnityEngine+Rendering+BatchDrawCommandFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchDrawCommandFlags {
    FlipWinding = 1i32,
    HasMotion = 2i32,
    HasSortingPosition = 8i32,
    IsLightMapped = 4i32,
    LODCrossFade = 16i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+Rendering+BatchDrawCommandFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchDrawCommandFlags =>
    "UnityEngine.Rendering"."BatchDrawCommandFlags"
);
