#[cfg(feature = "UnityEngine+XR+MeshVertexAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshVertexAttributes {
    #[default]
    Colors = 8i32,
    None = 0i32,
    Normals = 1i32,
    Tangents = 2i32,
    UVs = 4i32,
}
#[cfg(feature = "UnityEngine+XR+MeshVertexAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::MeshVertexAttributes =>
    "UnityEngine.XR"."MeshVertexAttributes"
);
