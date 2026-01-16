#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+LineInfo")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct LineInfo {
    pub controlCharacterCount: i32,
    pub characterCount: i32,
    pub visibleCharacterCount: i32,
    pub visibleSpaceCount: i32,
    pub spaceCount: i32,
    pub wordCount: i32,
    pub firstCharacterIndex: i32,
    pub firstVisibleCharacterIndex: i32,
    pub lastCharacterIndex: i32,
    pub lastVisibleCharacterIndex: i32,
    pub length: f32,
    pub lineHeight: f32,
    pub ascender: f32,
    pub baseline: f32,
    pub descender: f32,
    pub maxAdvance: f32,
    pub width: f32,
    pub marginLeft: f32,
    pub marginRight: f32,
    pub alignment: crate::UnityEngine::TextCore::Text::TextAlignment,
    pub lineExtents: crate::UnityEngine::TextCore::Text::Extents,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+LineInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::TextCore::Text::LineInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "LineInfo";
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+LineInfo")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::TextCore::Text::LineInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+LineInfo")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::TextCore::Text::LineInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+LineInfo")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::TextCore::Text::LineInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+LineInfo")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::TextCore::Text::LineInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+LineInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::TextCore::Text::LineInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+LineInfo")]
impl crate::UnityEngine::TextCore::Text::LineInfo {}
