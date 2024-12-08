#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+DataType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Angle = 9i32,
    Color = 5i32,
    CustomIdent = 10i32,
    Integer = 2i32,
    Length = 3i32,
    None = 0i32,
    Number = 1i32,
    Percentage = 4i32,
    Resource = 6i32,
    Time = 8i32,
    Url = 7i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+DataType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Syntax::DataType =>
    "UnityEngine.UIElements.StyleSheets.Syntax"."DataType"
);
