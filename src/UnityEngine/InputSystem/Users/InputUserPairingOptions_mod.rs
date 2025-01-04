#[cfg(feature = "UnityEngine+InputSystem+Users+InputUserPairingOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputUserPairingOptions {
    #[default]
    ForceNoPlatformUserAccountSelection = 2i32,
    ForcePlatformUserAccountSelection = 1i32,
    None = 0i32,
    UnpairCurrentDevicesFromUser = 8i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUserPairingOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Users::InputUserPairingOptions =>
    "UnityEngine.InputSystem.Users"."InputUserPairingOptions"
);
