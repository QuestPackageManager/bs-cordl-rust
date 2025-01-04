#[cfg(feature = "UnityEngine+UIElements+ScrollViewMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollViewMode {
    #[default]
    Horizontal = 1i32,
    Vertical = 0i32,
    VerticalAndHorizontal = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+ScrollViewMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ScrollViewMode =>
    "UnityEngine.UIElements"."ScrollViewMode"
);
