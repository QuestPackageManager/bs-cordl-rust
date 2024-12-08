#[cfg(feature = "UnityEngine+TextEditOp")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditOp {
    Backspace = 19i32,
    Cut = 23i32,
    Delete = 18i32,
    DeleteLineBack = 22i32,
    DeleteWordBack = 20i32,
    DeleteWordForward = 21i32,
    MoveDown = 3i32,
    MoveGraphicalLineEnd = 11i32,
    MoveGraphicalLineStart = 10i32,
    MoveLeft = 0i32,
    MoveLineEnd = 5i32,
    MoveLineStart = 4i32,
    MovePageDown = 9i32,
    MovePageUp = 8i32,
    MoveParagraphBackward = 15i32,
    MoveParagraphForward = 14i32,
    MoveRight = 1i32,
    MoveTextEnd = 7i32,
    MoveTextStart = 6i32,
    MoveToEndOfPreviousWord = 17i32,
    MoveToStartOfNextWord = 16i32,
    MoveUp = 2i32,
    MoveWordLeft = 12i32,
    MoveWordRight = 13i32,
    Paste = 24i32,
    ScrollEnd = 26i32,
    ScrollPageDown = 28i32,
    ScrollPageUp = 27i32,
    ScrollStart = 25i32,
}
#[cfg(feature = "UnityEngine+TextEditOp")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextEditOp => "UnityEngine"
    ."TextEditOp"
);
