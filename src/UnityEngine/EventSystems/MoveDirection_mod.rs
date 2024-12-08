#[cfg(feature = "UnityEngine+EventSystems+MoveDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Down = 3i32,
    Left = 0i32,
    None = 4i32,
    Right = 2i32,
    Up = 1i32,
}
#[cfg(feature = "UnityEngine+EventSystems+MoveDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::MoveDirection =>
    "UnityEngine.EventSystems"."MoveDirection"
);
