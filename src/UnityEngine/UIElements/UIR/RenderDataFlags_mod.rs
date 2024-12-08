#[cfg(feature = "UnityEngine+UIElements+UIR+RenderDataFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderDataFlags {
    IsIgnoringDynamicColorHint = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderDataFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::RenderDataFlags =>
    "UnityEngine.UIElements.UIR"."RenderDataFlags"
);
