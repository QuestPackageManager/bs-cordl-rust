#[cfg(feature = "UnityEngine+EventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    ContextClick = 16i32,
    DragExited = 15i32,
    DragPerform = 10i32,
    DragUpdated = 9i32,
    ExecuteCommand = 14i32,
    Ignore = 11i32,
    KeyDown = 4i32,
    KeyUp = 5i32,
    Layout = 8i32,
    MouseDown = 0i32,
    MouseDrag = 3i32,
    MouseEnterWindow = 20i32,
    MouseLeaveWindow = 21i32,
    MouseMove = 2i32,
    MouseUp = 1i32,
    Repaint = 7i32,
    ScrollWheel = 6i32,
    TouchDown = 30i32,
    TouchEnter = 33i32,
    TouchLeave = 34i32,
    TouchMove = 32i32,
    TouchStationary = 35i32,
    TouchUp = 31i32,
    Used = 12i32,
    ValidateCommand = 13i32,
}
#[cfg(feature = "UnityEngine+EventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventType => "UnityEngine"
    ."EventType"
);
