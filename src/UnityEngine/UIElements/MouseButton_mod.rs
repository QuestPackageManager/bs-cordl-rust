#[cfg(feature = "UnityEngine+UIElements+MouseButton")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MouseButton {
    #[default]
    LeftMouse = 0i32,
    MiddleMouse = 2i32,
    RightMouse = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+MouseButton")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MouseButton =>
    "UnityEngine.UIElements"."MouseButton"
);
