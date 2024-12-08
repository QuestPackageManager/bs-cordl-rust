#[cfg(feature = "UnityEngine+InputSystem+InputActionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputActionType {
    Button = 1i32,
    PassThrough = 2i32,
    Value = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputActionType =>
    "UnityEngine.InputSystem"."InputActionType"
);
