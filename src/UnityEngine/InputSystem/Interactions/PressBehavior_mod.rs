#[cfg(feature = "UnityEngine+InputSystem+Interactions+PressBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressBehavior {
    PressAndRelease = 2i32,
    PressOnly = 0i32,
    ReleaseOnly = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Interactions+PressBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Interactions::PressBehavior =>
    "UnityEngine.InputSystem.Interactions"."PressBehavior"
);
