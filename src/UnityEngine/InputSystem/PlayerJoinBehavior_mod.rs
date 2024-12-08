#[cfg(feature = "UnityEngine+InputSystem+PlayerJoinBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerJoinBehavior {
    JoinPlayersManually = 2i32,
    JoinPlayersWhenButtonIsPressed = 0i32,
    JoinPlayersWhenJoinActionIsTriggered = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerJoinBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::PlayerJoinBehavior =>
    "UnityEngine.InputSystem"."PlayerJoinBehavior"
);
