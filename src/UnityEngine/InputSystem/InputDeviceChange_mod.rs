#[cfg(feature = "UnityEngine+InputSystem+InputDeviceChange")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputDeviceChange {
    #[default]
    Added = 0i32,
    ConfigurationChanged = 7i32,
    Destroyed = 10i32,
    Disabled = 5i32,
    Disconnected = 2i32,
    Enabled = 4i32,
    HardReset = 9i32,
    Reconnected = 3i32,
    Removed = 1i32,
    SoftReset = 8i32,
    UsageChanged = 6i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputDeviceChange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputDeviceChange =>
    "UnityEngine.InputSystem"."InputDeviceChange"
);
