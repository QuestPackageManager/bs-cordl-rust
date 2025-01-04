#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionMultiplierType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExpressionMultiplierType {
    #[default]
    GroupAtLeastOne = 6i32,
    None = 0i32,
    OneOrMore = 2i32,
    OneOrMoreComma = 5i32,
    Ranges = 4i32,
    ZeroOrMore = 1i32,
    ZeroOrOne = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionMultiplierType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplierType =>
    "UnityEngine.UIElements.StyleSheets.Syntax"."ExpressionMultiplierType"
);
