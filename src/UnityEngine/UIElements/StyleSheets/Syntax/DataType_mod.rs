#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+DataType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum DataType {
    #[default]
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+DataType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::StyleSheets::Syntax::DataType
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets.Syntax";
    const CLASS_NAME: &'static str = "DataType";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+DataType")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::StyleSheets::Syntax::DataType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+DataType")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::StyleSheets::Syntax::DataType
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+DataType")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::StyleSheets::Syntax::DataType
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+Syntax+DataType")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::StyleSheets::Syntax::DataType
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
