#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleEnumType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleEnumType {
    Align = 0i32,
    BackgroundPositionKeyword = 1i32,
    BackgroundSizeType = 2i32,
    DisplayStyle = 3i32,
    EasingMode = 4i32,
    FlexDirection = 5i32,
    FontStyle = 6i32,
    Justify = 7i32,
    Overflow = 8i32,
    OverflowClipBox = 9i32,
    OverflowInternal = 10i32,
    Position = 11i32,
    Repeat = 12i32,
    RepeatXY = 13i32,
    ScaleMode = 14i32,
    TextAnchor = 15i32,
    TextOverflow = 16i32,
    TextOverflowPosition = 17i32,
    TransformOriginOffset = 18i32,
    Visibility = 19i32,
    WhiteSpace = 20i32,
    Wrap = 21i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleEnumType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StyleEnumType =>
    "UnityEngine.UIElements.StyleSheets"."StyleEnumType"
);
