#[cfg(feature = "UnityEngine+Rendering+RenderTextureSubElement")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderTextureSubElement {
    #[default]
    Color = 0i32,
    Default = 3i32,
    Depth = 1i32,
    Stencil = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+RenderTextureSubElement")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::RenderTextureSubElement
    => "UnityEngine.Rendering"."RenderTextureSubElement"
);
