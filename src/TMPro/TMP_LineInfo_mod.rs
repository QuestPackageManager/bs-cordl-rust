#[cfg(feature = "TMPro+TMP_LineInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TMP_LineInfo {
    pub controlCharacterCount: i32,
    pub characterCount: i32,
    pub visibleCharacterCount: i32,
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
    pub alignment: crate::TMPro::HorizontalAlignmentOptions,
    pub lineExtents: crate::TMPro::Extents,
}
#[cfg(feature = "TMPro+TMP_LineInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_LineInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_LineInfo";
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
#[cfg(feature = "TMPro+TMP_LineInfo")]
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::TMP_LineInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "TMPro+TMP_LineInfo")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::TMPro::TMP_LineInfo {
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
#[cfg(feature = "TMPro+TMP_LineInfo")]
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::TMP_LineInfo {
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
#[cfg(feature = "TMPro+TMP_LineInfo")]
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::TMP_LineInfo {
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
#[cfg(feature = "TMPro+TMP_LineInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_LineInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_LineInfo")]
impl crate::TMPro::TMP_LineInfo {}
