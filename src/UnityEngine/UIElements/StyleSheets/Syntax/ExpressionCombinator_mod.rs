#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionCombinator")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExpressionCombinator {
    #[default]
    AndAnd = 3i32,
    Group = 5i32,
    Juxtaposition = 4i32,
    None = 0i32,
    Or = 1i32,
    OrOr = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionCombinator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets.Syntax";
    const CLASS_NAME: &'static str = "ExpressionCombinator";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionCombinator")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionCombinator")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionCombinator")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionCombinator")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionCombinator {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
