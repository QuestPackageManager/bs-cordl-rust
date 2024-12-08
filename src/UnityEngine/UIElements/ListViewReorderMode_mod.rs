#[cfg(feature = "UnityEngine+UIElements+ListViewReorderMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListViewReorderMode {
    Animated = 1i32,
    Simple = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ListViewReorderMode =>
    "UnityEngine.UIElements"."ListViewReorderMode"
);
