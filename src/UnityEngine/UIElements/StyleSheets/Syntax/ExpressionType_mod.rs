#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionType {
    Combinator = 3i32,
    Data = 1i32,
    Keyword = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionType =>
    "UnityEngine.UIElements.StyleSheets.Syntax"."ExpressionType"
);
