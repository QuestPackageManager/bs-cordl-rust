#[cfg(feature = "UnityEngine+InputSystem+UI+UIPointerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UIPointerType {
    MouseOrPen = 1i32,
    None = 0i32,
    Touch = 2i32,
    Tracked = 3i32,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+UIPointerType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::UI::UIPointerType =>
    "UnityEngine.InputSystem.UI"."UIPointerType"
);
