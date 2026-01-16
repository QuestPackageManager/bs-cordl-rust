#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenType
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets.Syntax";
    const CLASS_NAME: &'static str = "StyleSyntaxTokenType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenType")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenType")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenType
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenType")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxTokenType")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenType
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
