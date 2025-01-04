#[cfg(feature = "UnityEngine+MeshColliderCookingOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshColliderCookingOptions {
    #[default]
    CookForFasterSimulation = 2i32,
    EnableMeshCleaning = 4i32,
    InflateConvexMesh = 1i32,
    None = 0i32,
    UseFastMidphase = 16i32,
    WeldColocatedVertices = 8i32,
}
#[cfg(feature = "UnityEngine+MeshColliderCookingOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::MeshColliderCookingOptions =>
    "UnityEngine"."MeshColliderCookingOptions"
);
