#[cfg(feature = "UnityEngine+InputSystem+TouchPhase")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TouchPhase {
    #[default]
    Began = 1i32,
    Canceled = 4i32,
    Ended = 3i32,
    Moved = 2i32,
    None = 0i32,
    Stationary = 5i32,
}
#[cfg(feature = "UnityEngine+InputSystem+TouchPhase")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::TouchPhase =>
    "UnityEngine.InputSystem"."TouchPhase"
);
