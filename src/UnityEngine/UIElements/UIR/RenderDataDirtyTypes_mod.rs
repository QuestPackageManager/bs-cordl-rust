#[cfg(feature = "UnityEngine+UIElements+UIR+RenderDataDirtyTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderDataDirtyTypes {
    AllVisuals = 112i32,
    ClipRectSize = 2i32,
    Clipping = 4i32,
    ClippingHierarchy = 8i32,
    Color = 512i32,
    None = 0i32,
    Opacity = 128i32,
    OpacityHierarchy = 256i32,
    Transform = 1i32,
    Visuals = 16i32,
    VisualsHierarchy = 32i32,
    VisualsOpacityId = 64i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderDataDirtyTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::RenderDataDirtyTypes => "UnityEngine.UIElements.UIR"
    ."RenderDataDirtyTypes"
);
