#[cfg(feature = "UnityEngine+InputSystem+InputControlLayoutChange")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputControlLayoutChange {
    #[default]
    Added = 0i32,
    Removed = 1i32,
    Replaced = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlLayoutChange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlLayoutChange => "UnityEngine.InputSystem"
    ."InputControlLayoutChange"
);
