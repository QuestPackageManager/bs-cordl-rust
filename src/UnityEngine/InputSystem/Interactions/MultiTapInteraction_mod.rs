#[cfg(feature = "UnityEngine+InputSystem+Interactions+MultiTapInteraction+TapPhase")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiTapInteraction_TapPhase {
    None = 0i32,
    WaitingForNextPress = 2i32,
    WaitingForNextRelease = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Interactions+MultiTapInteraction+TapPhase")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Interactions::MultiTapInteraction_TapPhase =>
    "UnityEngine.InputSystem.Interactions"."MultiTapInteraction/TapPhase"
);
