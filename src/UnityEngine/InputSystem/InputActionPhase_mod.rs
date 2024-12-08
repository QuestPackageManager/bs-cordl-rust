#[cfg(feature = "UnityEngine+InputSystem+InputActionPhase")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputActionPhase {
    Canceled = 4i32,
    Disabled = 0i32,
    Performed = 3i32,
    Started = 2i32,
    Waiting = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionPhase")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputActionPhase =>
    "UnityEngine.InputSystem"."InputActionPhase"
);
