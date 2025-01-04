#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdatePhase")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VisualTreeUpdatePhase {
    #[default]
    Animation = 2i32,
    Bindings = 1i32,
    Count = 7i32,
    Layout = 4i32,
    Repaint = 6i32,
    Styles = 3i32,
    TransformClip = 5i32,
    ViewData = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdatePhase")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualTreeUpdatePhase
    => "UnityEngine.UIElements"."VisualTreeUpdatePhase"
);
