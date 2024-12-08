#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdateType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputUpdateType {
    BeforeRender = 4i32,
    Default = 11i32,
    Dynamic = 1i32,
    Editor = 8i32,
    Fixed = 2i32,
    Manual = 16i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdateType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputUpdateType =>
    "UnityEngine.InputSystem.LowLevel"."InputUpdateType"
);
