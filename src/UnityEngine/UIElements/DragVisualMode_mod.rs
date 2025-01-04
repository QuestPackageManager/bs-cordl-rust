#[cfg(feature = "UnityEngine+UIElements+DragVisualMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DragVisualMode {
    #[default]
    Copy = 1i32,
    Move = 2i32,
    None = 0i32,
    Rejected = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+DragVisualMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DragVisualMode =>
    "UnityEngine.UIElements"."DragVisualMode"
);
