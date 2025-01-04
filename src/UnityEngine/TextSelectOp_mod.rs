#[cfg(feature = "UnityEngine+TextSelectOp")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextSelectOp {
    #[default]
    Copy = 18i32,
    ExpandSelectGraphicalLineEnd = 9i32,
    ExpandSelectGraphicalLineStart = 8i32,
    SelectAll = 19i32,
    SelectDown = 3i32,
    SelectGraphicalLineEnd = 11i32,
    SelectGraphicalLineStart = 10i32,
    SelectLeft = 0i32,
    SelectNone = 20i32,
    SelectPageDown = 7i32,
    SelectPageUp = 6i32,
    SelectParagraphBackward = 16i32,
    SelectParagraphForward = 17i32,
    SelectRight = 1i32,
    SelectTextEnd = 5i32,
    SelectTextStart = 4i32,
    SelectToEndOfPreviousWord = 14i32,
    SelectToStartOfNextWord = 15i32,
    SelectUp = 2i32,
    SelectWordLeft = 12i32,
    SelectWordRight = 13i32,
}
#[cfg(feature = "UnityEngine+TextSelectOp")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextSelectOp => "UnityEngine"
    ."TextSelectOp"
);
