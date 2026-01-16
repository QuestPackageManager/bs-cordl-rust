#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleEnumType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum StyleEnumType {
    #[default]
    Align = 0i32,
    Axis = 1i32,
    BackgroundPositionKeyword = 2i32,
    BackgroundSizeType = 3i32,
    DisplayStyle = 4i32,
    EasingMode = 5i32,
    EditorTextRenderingMode = 6i32,
    FlexDirection = 7i32,
    FontStyle = 8i32,
    Justify = 9i32,
    Overflow = 10i32,
    OverflowClipBox = 11i32,
    OverflowInternal = 12i32,
    Position = 13i32,
    Repeat = 14i32,
    RepeatXY = 15i32,
    ScaleMode = 16i32,
    TextAnchor = 17i32,
    TextGeneratorType = 18i32,
    TextOverflow = 19i32,
    TextOverflowPosition = 20i32,
    TransformOriginOffset = 21i32,
    Visibility = 22i32,
    WhiteSpace = 23i32,
    Wrap = 24i32,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleEnumType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::StyleSheets::StyleEnumType
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets";
    const CLASS_NAME: &'static str = "StyleEnumType";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleEnumType")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::StyleSheets::StyleEnumType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleEnumType")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::StyleSheets::StyleEnumType
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleEnumType")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::StyleSheets::StyleEnumType
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleEnumType")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::StyleSheets::StyleEnumType
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
