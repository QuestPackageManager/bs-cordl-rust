#[cfg(feature = "UnityEngine+InputSystem+PenButton")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PenButton {
    #[default]
    Barrel1 = 2i32,
    Barrel2 = 3i32,
    Barrel3 = 5i32,
    Barrel4 = 6i32,
    Eraser = 1i32,
    InRange = 4i32,
    Tip = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+PenButton")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::PenButton =>
    "UnityEngine.InputSystem"."PenButton"
);
