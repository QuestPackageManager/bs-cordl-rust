#[cfg(feature = "UnityEngine+UIElements+PanelScaleMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelScaleMode {
    #[default]
    ConstantPhysicalSize = 1i32,
    ConstantPixelSize = 0i32,
    ScaleWithScreenSize = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+PanelScaleMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PanelScaleMode =>
    "UnityEngine.UIElements"."PanelScaleMode"
);
