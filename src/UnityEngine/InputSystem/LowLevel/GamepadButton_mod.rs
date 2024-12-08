#[cfg(feature = "UnityEngine+InputSystem+LowLevel+GamepadButton")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadButton {
    A = 6i32,
    B = 5i32,
    DpadDown = 1i32,
    DpadLeft = 2i32,
    DpadRight = 3i32,
    DpadUp = 0i32,
    LeftShoulder = 10i32,
    LeftStick = 8i32,
    LeftTrigger = 32i32,
    North = 4i32,
    RightShoulder = 11i32,
    RightStick = 9i32,
    RightTrigger = 33i32,
    Select = 13i32,
    Square = 7i32,
    Start = 12i32,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+GamepadButton")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::GamepadButton =>
    "UnityEngine.InputSystem.LowLevel"."GamepadButton"
);
