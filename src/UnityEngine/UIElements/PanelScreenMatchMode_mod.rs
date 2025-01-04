#[cfg(feature = "UnityEngine+UIElements+PanelScreenMatchMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelScreenMatchMode {
    #[default]
    Expand = 2i32,
    MatchWidthOrHeight = 0i32,
    Shrink = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+PanelScreenMatchMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PanelScreenMatchMode =>
    "UnityEngine.UIElements"."PanelScreenMatchMode"
);
