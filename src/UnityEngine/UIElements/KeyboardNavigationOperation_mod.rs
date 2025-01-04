#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationOperation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyboardNavigationOperation {
    #[default]
    Begin = 10i32,
    Cancel = 2i32,
    End = 11i32,
    MoveLeft = 7i32,
    MoveRight = 6i32,
    Next = 5i32,
    None = 0i32,
    PageDown = 9i32,
    PageUp = 8i32,
    Previous = 4i32,
    SelectAll = 1i32,
    Submit = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationOperation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::KeyboardNavigationOperation => "UnityEngine.UIElements"
    ."KeyboardNavigationOperation"
);
