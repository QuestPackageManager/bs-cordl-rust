#[cfg(feature = "UnityEngine+TextCore+Text+WordWrapState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WordWrapState {
    pub previousWordBreak: i32,
    pub totalCharacterCount: i32,
    pub visibleCharacterCount: i32,
    pub visibleSpaceCount: i32,
    pub visibleSpriteCount: i32,
    pub visibleLinkCount: i32,
    pub firstCharacterIndex: i32,
    pub firstVisibleCharacterIndex: i32,
    pub lastCharacterIndex: i32,
    pub lastVisibleCharIndex: i32,
    pub lineNumber: i32,
    pub maxCapHeight: f32,
    pub maxAscender: f32,
    pub maxDescender: f32,
    pub maxLineAscender: f32,
    pub maxLineDescender: f32,
    pub startOfLineAscender: f32,
    pub xAdvance: f32,
    pub preferredWidth: f32,
    pub preferredHeight: f32,
    pub previousLineScale: f32,
    pub pageAscender: f32,
    pub wordCount: i32,
    pub fontStyle: crate::UnityEngine::TextCore::Text::FontStyles,
    pub fontScale: f32,
    pub fontScaleMultiplier: f32,
    pub italicAngle: i32,
    pub currentFontSize: f32,
    pub baselineOffset: f32,
    pub lineOffset: f32,
    pub textInfo: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextInfo,
    >,
    pub lineInfo: crate::UnityEngine::TextCore::Text::LineInfo,
    pub vertexColor: crate::UnityEngine::Color32,
    pub underlineColor: crate::UnityEngine::Color32,
    pub strikethroughColor: crate::UnityEngine::Color32,
    pub highlightColor: crate::UnityEngine::Color32,
    pub highlightState: crate::UnityEngine::TextCore::Text::HighlightState,
    pub basicStyleStack: crate::UnityEngine::TextCore::Text::FontStyleStack,
    pub italicAngleStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
    pub colorStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub underlineColorStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub strikethroughColorStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub highlightColorStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub highlightStateStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::TextCore::Text::HighlightState,
    >,
    pub colorGradientStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextColorGradient>,
    >,
    pub sizeStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<f32>,
    pub indentStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<f32>,
    pub fontWeightStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::TextCore::Text::TextFontWeight,
    >,
    pub styleStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
    pub baselineStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<f32>,
    pub actionStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
    pub materialReferenceStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::TextCore::Text::MaterialReference,
    >,
    pub lineJustificationStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::TextCore::Text::TextAlignment,
    >,
    pub lastBaseGlyphIndex: i32,
    pub spriteAnimationId: i32,
    pub currentFontAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::FontAsset,
    >,
    pub currentSpriteAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::SpriteAsset,
    >,
    pub currentMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub currentMaterialIndex: i32,
    pub meshExtents: crate::UnityEngine::TextCore::Text::Extents,
    pub tagNoParsing: bool,
    pub isNonBreakingSpace: bool,
    pub isDrivenLineSpacing: bool,
    pub fxScale: crate::UnityEngine::Vector3,
    pub fxRotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "UnityEngine+TextCore+Text+WordWrapState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::WordWrapState =>
    "UnityEngine.TextCore.Text"."WordWrapState"
);
#[cfg(feature = "UnityEngine+TextCore+Text+WordWrapState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::WordWrapState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+WordWrapState")]
impl crate::UnityEngine::TextCore::Text::WordWrapState {}
