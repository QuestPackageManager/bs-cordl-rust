#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StyleSyntaxTokenType {
    #[default]
    Asterisk = 9i32,
    CloseBrace = 17i32,
    CloseBracket = 15i32,
    Comma = 7i32,
    DoubleAmpersand = 6i32,
    DoubleBar = 5i32,
    End = 20i32,
    ExclamationPoint = 13i32,
    GreaterThan = 19i32,
    HashMark = 12i32,
    LessThan = 18i32,
    Number = 2i32,
    OpenBrace = 16i32,
    OpenBracket = 14i32,
    Plus = 10i32,
    QuestionMark = 11i32,
    SingleBar = 4i32,
    SingleQuote = 8i32,
    Space = 3i32,
    String = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenType =>
    "UnityEngine.UIElements.StyleSheets.Syntax"."StyleSyntaxTokenType"
);
