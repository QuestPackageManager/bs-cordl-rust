#[cfg(feature = "UnityEngine+UIElements+StyleSheets+MatchResultErrorCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatchResultErrorCode {
    #[default]
    EmptyValue = 2i32,
    ExpectedEndOfValue = 3i32,
    None = 0i32,
    Syntax = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+MatchResultErrorCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::MatchResultErrorCode =>
    "UnityEngine.UIElements.StyleSheets"."MatchResultErrorCode"
);
