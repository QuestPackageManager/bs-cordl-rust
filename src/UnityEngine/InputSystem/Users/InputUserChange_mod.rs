#[cfg(feature = "UnityEngine+InputSystem+Users+InputUserChange")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputUserChange {
    #[default]
    AccountChanged = 6i32,
    AccountNameChanged = 7i32,
    AccountSelectionCanceled = 9i32,
    AccountSelectionComplete = 10i32,
    AccountSelectionInProgress = 8i32,
    Added = 0i32,
    ControlSchemeChanged = 11i32,
    ControlsChanged = 12i32,
    DeviceLost = 4i32,
    DevicePaired = 2i32,
    DeviceRegained = 5i32,
    DeviceUnpaired = 3i32,
    Removed = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUserChange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Users::InputUserChange
    => "UnityEngine.InputSystem.Users"."InputUserChange"
);
