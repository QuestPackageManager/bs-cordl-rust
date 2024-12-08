#[cfg(feature = "UnityEngine+InputSystem+PlayerNotifications")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerNotifications {
    BroadcastMessages = 1i32,
    InvokeCSharpEvents = 3i32,
    InvokeUnityEvents = 2i32,
    SendMessages = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerNotifications")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::PlayerNotifications =>
    "UnityEngine.InputSystem"."PlayerNotifications"
);
