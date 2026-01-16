#[cfg(feature = "cordl_class_TMPro+TMP_CharacterInfo")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct TMP_CharacterInfo {
    pub elementType: crate::TMPro::TMP_TextElementType,
    pub character: char,
    pub index: i32,
    pub stringLength: i32,
    pub textElement: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_TextElement>,
    pub alternativeGlyph: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
    pub fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub materialReferenceIndex: i32,
    pub isUsingAlternateTypeface: bool,
    pub pointSize: f32,
    pub lineNumber: i32,
    pub pageNumber: i32,
    pub vertexIndex: i32,
    pub vertex_BL: crate::TMPro::TMP_Vertex,
    pub vertex_TL: crate::TMPro::TMP_Vertex,
    pub vertex_TR: crate::TMPro::TMP_Vertex,
    pub vertex_BR: crate::TMPro::TMP_Vertex,
    pub topLeft: crate::UnityEngine::Vector3,
    pub bottomLeft: crate::UnityEngine::Vector3,
    pub topRight: crate::UnityEngine::Vector3,
    pub bottomRight: crate::UnityEngine::Vector3,
    pub origin: f32,
    pub xAdvance: f32,
    pub ascender: f32,
    pub baseLine: f32,
    pub descender: f32,
    pub adjustedAscender: f32,
    pub adjustedDescender: f32,
    pub adjustedHorizontalAdvance: f32,
    pub aspectRatio: f32,
    pub scale: f32,
    pub color: crate::UnityEngine::Color32,
    pub underlineColor: crate::UnityEngine::Color32,
    pub underlineVertexIndex: i32,
    pub strikethroughColor: crate::UnityEngine::Color32,
    pub strikethroughVertexIndex: i32,
    pub highlightColor: crate::UnityEngine::Color32,
    pub highlightState: crate::TMPro::HighlightState,
    pub style: crate::TMPro::FontStyles,
    pub isVisible: bool,
}
#[cfg(feature = "cordl_class_TMPro+TMP_CharacterInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_CharacterInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_CharacterInfo";
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
#[cfg(feature = "cordl_class_TMPro+TMP_CharacterInfo")]
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::TMP_CharacterInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_CharacterInfo")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::TMPro::TMP_CharacterInfo {
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
#[cfg(feature = "cordl_class_TMPro+TMP_CharacterInfo")]
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::TMP_CharacterInfo {
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
#[cfg(feature = "cordl_class_TMPro+TMP_CharacterInfo")]
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::TMP_CharacterInfo {
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
#[cfg(feature = "cordl_class_TMPro+TMP_CharacterInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_CharacterInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_CharacterInfo")]
impl crate::TMPro::TMP_CharacterInfo {}
