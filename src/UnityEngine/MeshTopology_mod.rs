#[cfg(feature = "UnityEngine+MeshTopology")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshTopology {
    #[default]
    LineStrip = 4i32,
    Lines = 3i32,
    Points = 5i32,
    Quads = 2i32,
    Triangles = 0i32,
}
#[cfg(feature = "UnityEngine+MeshTopology")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::MeshTopology => "UnityEngine"
    ."MeshTopology"
);
