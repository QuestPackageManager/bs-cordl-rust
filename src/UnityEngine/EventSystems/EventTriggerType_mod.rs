#[cfg(feature = "UnityEngine+EventSystems+EventTriggerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventTriggerType {
    #[default]
    BeginDrag = 13i32,
    Cancel = 16i32,
    Deselect = 10i32,
    Drag = 5i32,
    Drop = 6i32,
    EndDrag = 14i32,
    InitializePotentialDrag = 12i32,
    Move = 11i32,
    PointerClick = 4i32,
    PointerDown = 2i32,
    PointerEnter = 0i32,
    PointerExit = 1i32,
    PointerUp = 3i32,
    Scroll = 7i32,
    Select = 9i32,
    Submit = 15i32,
    UpdateSelected = 8i32,
}
#[cfg(feature = "UnityEngine+EventSystems+EventTriggerType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::EventTriggerType =>
    "UnityEngine.EventSystems"."EventTriggerType"
);
