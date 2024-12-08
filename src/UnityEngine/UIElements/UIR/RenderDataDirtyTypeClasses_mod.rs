#[cfg(feature = "UnityEngine+UIElements+UIR+RenderDataDirtyTypeClasses")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderDataDirtyTypeClasses {
    Clipping = 0i32,
    Color = 2i32,
    Count = 5i32,
    Opacity = 1i32,
    TransformSize = 3i32,
    Visuals = 4i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderDataDirtyTypeClasses")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::RenderDataDirtyTypeClasses =>
    "UnityEngine.UIElements.UIR"."RenderDataDirtyTypeClasses"
);
