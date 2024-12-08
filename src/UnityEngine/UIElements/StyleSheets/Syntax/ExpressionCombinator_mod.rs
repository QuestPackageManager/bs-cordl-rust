#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionCombinator")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionCombinator {
    AndAnd = 3i32,
    Group = 5i32,
    Juxtaposition = 4i32,
    None = 0i32,
    Or = 1i32,
    OrOr = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionCombinator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator =>
    "UnityEngine.UIElements.StyleSheets.Syntax"."ExpressionCombinator"
);
