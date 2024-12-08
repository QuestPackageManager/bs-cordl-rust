#[cfg(feature = "UnityEngine+RenderingPath")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingPath {
    DeferredLighting = 2i32,
    DeferredShading = 3i32,
    Forward = 1i32,
    UsePlayerSettings = -1i32,
    VertexLit = 0i32,
}
#[cfg(feature = "UnityEngine+RenderingPath")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderingPath => "UnityEngine"
    ."RenderingPath"
);
