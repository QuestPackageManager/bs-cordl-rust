#[cfg(feature = "UnityEngine+InputSystem+LowLevel+MouseButton")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MouseButton {
    #[default]
    Back = 4i32,
    Forward = 3i32,
    Left = 0i32,
    Middle = 2i32,
    Right = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+MouseButton")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::MouseButton
    => "UnityEngine.InputSystem.LowLevel"."MouseButton"
);
