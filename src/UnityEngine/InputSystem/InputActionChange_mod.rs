#[cfg(feature = "UnityEngine+InputSystem+InputActionChange")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputActionChange {
    #[default]
    ActionCanceled = 6i32,
    ActionDisabled = 1i32,
    ActionEnabled = 0i32,
    ActionMapDisabled = 3i32,
    ActionMapEnabled = 2i32,
    ActionPerformed = 5i32,
    ActionStarted = 4i32,
    BoundControlsAboutToChange = 7i32,
    BoundControlsChanged = 8i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionChange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputActionChange =>
    "UnityEngine.InputSystem"."InputActionChange"
);
