#[cfg(feature = "UnityEngine+RenderTextureMemoryless")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderTextureMemoryless {
    #[default]
    Color = 1i32,
    Depth = 2i32,
    MSAA = 4i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+RenderTextureMemoryless")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderTextureMemoryless =>
    "UnityEngine"."RenderTextureMemoryless"
);
