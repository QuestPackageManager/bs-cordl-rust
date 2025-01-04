#[cfg(feature = "UnityEngine+UIElements+EventCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventCategory {
    #[default]
    Bind = 9i32,
    ChangePanel = 11i32,
    ChangeValue = 8i32,
    Command = 14i32,
    Default = 0i32,
    EnterLeave = 3i32,
    EnterLeaveWindow = 4i32,
    Focus = 10i32,
    Geometry = 6i32,
    IMGUI = 16i32,
    Keyboard = 5i32,
    Navigation = 13i32,
    Pointer = 1i32,
    PointerMove = 2i32,
    Reserved = 31i32,
    Style = 7i32,
    StyleTransition = 12i32,
    Tooltip = 15i32,
}
#[cfg(feature = "UnityEngine+UIElements+EventCategory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventCategory =>
    "UnityEngine.UIElements"."EventCategory"
);
