#[cfg(feature = "UnityEngine+TextCore+Text+TextElementInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextElementInfo {
    pub character: char,
    pub index: i32,
    pub elementType: crate::UnityEngine::TextCore::Text::TextElementType,
    pub stringLength: i32,
    pub textElement: *mut crate::UnityEngine::TextCore::Text::TextElement,
    pub alternativeGlyph: *mut crate::UnityEngine::TextCore::Glyph,
    pub fontAsset: *mut crate::UnityEngine::TextCore::Text::FontAsset,
    pub spriteAsset: *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
    pub spriteIndex: i32,
    pub material: *mut crate::UnityEngine::Material,
    pub materialReferenceIndex: i32,
    pub isUsingAlternateTypeface: bool,
    pub pointSize: f32,
    pub lineNumber: i32,
    pub pageNumber: i32,
    pub vertexIndex: i32,
    pub vertexTopLeft: crate::UnityEngine::TextCore::Text::TextVertex,
    pub vertexBottomLeft: crate::UnityEngine::TextCore::Text::TextVertex,
    pub vertexTopRight: crate::UnityEngine::TextCore::Text::TextVertex,
    pub vertexBottomRight: crate::UnityEngine::TextCore::Text::TextVertex,
    pub topLeft: crate::UnityEngine::Vector3,
    pub bottomLeft: crate::UnityEngine::Vector3,
    pub topRight: crate::UnityEngine::Vector3,
    pub bottomRight: crate::UnityEngine::Vector3,
    pub origin: f32,
    pub ascender: f32,
    pub baseLine: f32,
    pub descender: f32,
    pub adjustedAscender: f32,
    pub adjustedDescender: f32,
    pub adjustedHorizontalAdvance: f32,
    pub xAdvance: f32,
    pub aspectRatio: f32,
    pub scale: f32,
    pub color: crate::UnityEngine::Color32,
    pub underlineColor: crate::UnityEngine::Color32,
    pub underlineVertexIndex: i32,
    pub strikethroughColor: crate::UnityEngine::Color32,
    pub strikethroughVertexIndex: i32,
    pub highlightColor: crate::UnityEngine::Color32,
    pub highlightState: crate::UnityEngine::TextCore::Text::HighlightState,
    pub style: crate::UnityEngine::TextCore::Text::FontStyles,
    pub isVisible: bool,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextElementInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextElementInfo =>
    "UnityEngine.TextCore.Text"."TextElementInfo"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextElementInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::TextElementInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextElementInfo")]
impl crate::UnityEngine::TextCore::Text::TextElementInfo {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
