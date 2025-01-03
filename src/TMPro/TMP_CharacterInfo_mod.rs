#[cfg(feature = "TMPro+TMP_CharacterInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TMP_CharacterInfo {
    pub character: char,
    pub index: i32,
    pub stringLength: i32,
    pub elementType: crate::TMPro::TMP_TextElementType,
    pub textElement: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_TextElement>,
    pub fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    pub spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    pub spriteIndex: i32,
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
#[cfg(feature = "TMPro+TMP_CharacterInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_CharacterInfo => "TMPro"
    ."TMP_CharacterInfo"
);
#[cfg(feature = "TMPro+TMP_CharacterInfo")]
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
