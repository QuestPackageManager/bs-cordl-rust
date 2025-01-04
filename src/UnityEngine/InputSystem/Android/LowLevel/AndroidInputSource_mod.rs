#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidInputSource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AndroidInputSource {
    #[default]
    Dpad = 513i32,
    Gamepad = 1025i32,
    Joystick = 16777232i32,
    Keyboard = 257i32,
    Mouse = 8194i32,
    Stylus = 16386i32,
    Touchpad = 1048584i32,
    Touchscreen = 4098i32,
    Trackball = 65540i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidInputSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidInputSource =>
    "UnityEngine.InputSystem.Android.LowLevel"."AndroidInputSource"
);
