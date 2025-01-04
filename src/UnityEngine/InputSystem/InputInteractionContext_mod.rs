#[cfg(feature = "UnityEngine+InputSystem+InputInteractionContext+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputInteractionContext_Flags {
    #[default]
    TimerHasExpired = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputInteractionContext+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputInteractionContext_Flags =>
    "UnityEngine.InputSystem"."InputInteractionContext/Flags"
);
