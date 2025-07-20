#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct TextGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_TextBackingArray: crate::UnityEngine::TextCore::Text::TextBackingContainer,
    pub m_TextProcessingArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::TextCore::Text::TextProcessingElement,
        >,
    >,
    pub m_InternalTextProcessingArraySize: i32,
    pub m_VertexBufferAutoSizeReduction: bool,
    pub m_HtmlTag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub m_HighlightState: crate::UnityEngine::TextCore::Text::HighlightState,
    pub m_IsIgnoringAlignment: bool,
    pub m_RectTransformCorners: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub m_MarginWidth: f32,
    pub m_MarginHeight: f32,
    pub m_PreferredWidth: f32,
    pub m_PreferredHeight: f32,
    pub m_CurrentFontAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::FontAsset,
    >,
    pub m_CurrentMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_CurrentMaterialIndex: i32,
    pub m_MaterialReferenceStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::TextCore::Text::MaterialReference,
    >,
    pub m_Padding: f32,
    pub m_CurrentSpriteAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::SpriteAsset,
    >,
    pub m_TotalCharacterCount: i32,
    pub m_FontSize: f32,
    pub m_FontScaleMultiplier: f32,
    pub m_CurrentFontSize: f32,
    pub m_SizeStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<f32>,
    pub m_TextStyleStacks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
        >,
    >,
    pub m_TextStyleStackDepth: i32,
    pub m_FontStyleInternal: crate::UnityEngine::TextCore::Text::FontStyles,
    pub m_FontStyleStack: crate::UnityEngine::TextCore::Text::FontStyleStack,
    pub m_FontWeightInternal: crate::UnityEngine::TextCore::Text::TextFontWeight,
    pub m_FontWeightStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::TextCore::Text::TextFontWeight,
    >,
    pub m_LineJustification: crate::UnityEngine::TextCore::Text::TextAlignment,
    pub m_LineJustificationStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::TextCore::Text::TextAlignment,
    >,
    pub m_BaselineOffset: f32,
    pub m_BaselineOffsetStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        f32,
    >,
    pub m_FontColor32: crate::UnityEngine::Color32,
    pub m_HtmlColor: crate::UnityEngine::Color32,
    pub m_UnderlineColor: crate::UnityEngine::Color32,
    pub m_StrikethroughColor: crate::UnityEngine::Color32,
    pub m_ColorStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub m_UnderlineColorStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub m_StrikethroughColorStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub m_HighlightColorStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub m_HighlightStateStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::TextCore::Text::HighlightState,
    >,
    pub m_ItalicAngleStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        i32,
    >,
    pub m_ColorGradientPreset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextColorGradient,
    >,
    pub m_ColorGradientStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextColorGradient>,
    >,
    pub m_ColorGradientPresetIsTinted: bool,
    pub m_ActionStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
    pub m_LineOffset: f32,
    pub m_LineHeight: f32,
    pub m_IsDrivenLineSpacing: bool,
    pub m_CSpacing: f32,
    pub m_MonoSpacing: f32,
    pub m_XAdvance: f32,
    pub m_TagLineIndent: f32,
    pub m_TagIndent: f32,
    pub m_IndentStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<f32>,
    pub m_TagNoParsing: bool,
    pub m_CharacterCount: i32,
    pub m_FirstCharacterOfLine: i32,
    pub m_LastCharacterOfLine: i32,
    pub m_FirstVisibleCharacterOfLine: i32,
    pub m_LastVisibleCharacterOfLine: i32,
    pub m_MaxLineAscender: f32,
    pub m_MaxLineDescender: f32,
    pub m_LineNumber: i32,
    pub m_LineVisibleCharacterCount: i32,
    pub m_LineVisibleSpaceCount: i32,
    pub m_FirstOverflowCharacterIndex: i32,
    pub m_PageNumber: i32,
    pub m_MarginLeft: f32,
    pub m_MarginRight: f32,
    pub m_Width: f32,
    pub m_MeshExtents: crate::UnityEngine::TextCore::Text::Extents,
    pub m_MaxCapHeight: f32,
    pub m_MaxAscender: f32,
    pub m_MaxDescender: f32,
    pub m_IsNewPage: bool,
    pub m_IsNonBreakingSpace: bool,
    pub m_SavedWordWrapState: crate::UnityEngine::TextCore::Text::WordWrapState,
    pub m_SavedLineState: crate::UnityEngine::TextCore::Text::WordWrapState,
    pub m_SavedEllipsisState: crate::UnityEngine::TextCore::Text::WordWrapState,
    pub m_SavedLastValidState: crate::UnityEngine::TextCore::Text::WordWrapState,
    pub m_SavedSoftLineBreakState: crate::UnityEngine::TextCore::Text::WordWrapState,
    pub m_TextElementType: crate::UnityEngine::TextCore::Text::TextElementType,
    pub m_isTextLayoutPhase: bool,
    pub m_SpriteIndex: i32,
    pub m_SpriteColor: crate::UnityEngine::Color32,
    pub m_CachedTextElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextElement,
    >,
    pub m_HighlightColor: crate::UnityEngine::Color32,
    pub m_CharWidthAdjDelta: f32,
    pub m_MaxFontSize: f32,
    pub m_MinFontSize: f32,
    pub m_AutoSizeIterationCount: i32,
    pub m_AutoSizeMaxIterationCount: i32,
    pub m_IsAutoSizePointSizeSet: bool,
    pub m_StartOfLineAscender: f32,
    pub m_LineSpacingDelta: f32,
    pub m_MaterialReferences: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::TextCore::Text::MaterialReference,
        >,
    >,
    pub m_SpriteCount: i32,
    pub m_StyleStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
    pub m_EllipsisInsertionCandidateStack: crate::UnityEngine::TextCore::Text::TextProcessingStack_1<
        crate::UnityEngine::TextCore::Text::WordWrapState,
    >,
    pub m_SpriteAnimationId: i32,
    pub m_ItalicAngle: i32,
    pub m_FXScale: crate::UnityEngine::Vector3,
    pub m_FXRotation: crate::UnityEngine::Quaternion,
    pub m_LastBaseGlyphIndex: i32,
    pub m_PageAscender: f32,
    pub m_XmlAttribute: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::TextCore::Text::RichTextTagAttribute,
        >,
    >,
    pub m_AttributeParameterValues: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub m_MaterialReferenceIndexLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<i32, i32>,
    >,
    pub m_IsCalculatingPreferredValues: bool,
    pub m_DefaultSpriteAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::SpriteAsset,
    >,
    pub m_TintSprite: bool,
    pub m_Ellipsis: crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter,
    pub m_Underline: crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter,
    pub m_InternalTextElementInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::TextCore::Text::TextElementInfo,
        >,
    >,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::TextGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextGenerator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator")]
impl crate::UnityEngine::TextCore::Text::TextGenerator {
    #[cfg(
        feature = "UnityEngine+TextCore+Text+TextGenerator+MissingCharacterEventCallback"
    )]
    pub type MissingCharacterEventCallback = crate::UnityEngine::TextCore::Text::TextGenerator_MissingCharacterEventCallback;
    #[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+SpecialCharacter")]
    pub type SpecialCharacter = crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter;
    pub fn CalculatePreferredValues(
        &mut self,
        fontSize: quest_hook::libil2cpp::ByRefMut<f32>,
        marginSize: crate::UnityEngine::Vector2,
        isTextAutoSizingEnabled: bool,
        textWrapMode: crate::UnityEngine::TextCore::Text::TextWrappingMode,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<f32>,
                    crate::UnityEngine::Vector2,
                    bool,
                    crate::UnityEngine::TextCore::Text::TextWrappingMode,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                crate::UnityEngine::Vector2,
                6usize,
            >("CalculatePreferredValues")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "CalculatePreferredValues",
                    6usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        fontSize,
                        marginSize,
                        isTextAutoSizingEnabled,
                        textWrapMode,
                        generationSettings,
                        textInfo,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearMarkupTagAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ClearMarkupTagAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "ClearMarkupTagAttributes",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearMesh(
        updateMesh: bool,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ClearMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "ClearMesh", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (updateMesh, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeMarginSize(
        &mut self,
        rect: crate::UnityEngine::Rect,
        margins: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rect, crate::UnityEngine::Vector4),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ComputeMarginSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "ComputeMarginSize", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rect, margins))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoMissingGlyphCallback(
        &mut self,
        unicode: u32,
        stringIndex: i32,
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::FontAsset,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("DoMissingGlyphCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "DoMissingGlyphCallback",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (unicode, stringIndex, fontAsset, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawTextHighlight(
        &mut self,
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        highlightColor: crate::UnityEngine::Color32,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Color32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("DrawTextHighlight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "DrawTextHighlight", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (start, end, highlightColor, generationSettings, textInfo),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawUnderlineMesh(
        &mut self,
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        startScale: f32,
        endScale: f32,
        maxScale: f32,
        sdfScale: f32,
        underlineColor: crate::UnityEngine::Color32,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    f32,
                    f32,
                    f32,
                    f32,
                    crate::UnityEngine::Color32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                9usize,
            >("DrawUnderlineMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "DrawUnderlineMesh", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        start,
                        end,
                        startScale,
                        endScale,
                        maxScale,
                        sdfScale,
                        underlineColor,
                        generationSettings,
                        textInfo,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateText(
        settings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GenerateText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateText", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (settings, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTextMesh(
        &mut self,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GenerateTextMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateTextMesh", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (generationSettings, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEllipsisSpecialCharacter(
        &mut self,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetEllipsisSpecialCharacter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetEllipsisSpecialCharacter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (generationSettings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPreferredValues(
        settings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                crate::UnityEngine::Vector2,
                2usize,
            >("GetPreferredValues")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "GetPreferredValues", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (settings, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPreferredValuesInternal(
        &mut self,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                crate::UnityEngine::Vector2,
                2usize,
            >("GetPreferredValuesInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetPreferredValuesInternal", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, (generationSettings, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSpecialCharacters(
        &mut self,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetSpecialCharacters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "GetSpecialCharacters",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (generationSettings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextElement(
        &mut self,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        unicode: u32,
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
        fontStyle: crate::UnityEngine::TextCore::Text::FontStyles,
        fontWeight: crate::UnityEngine::TextCore::Text::TextFontWeight,
        isUsingAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    u32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::FontAsset,
                    >,
                    crate::UnityEngine::TextCore::Text::FontStyles,
                    crate::UnityEngine::TextCore::Text::TextFontWeight,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextElement,
                >,
                6usize,
            >("GetTextElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "GetTextElement", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextElement,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        generationSettings,
                        unicode,
                        fontAsset,
                        fontStyle,
                        fontWeight,
                        isUsingAlternativeTypeface,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextGenerator() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextGenerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextGenerator,
                >,
                0usize,
            >("GetTextGenerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "GetTextGenerator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerator,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnderlineSpecialCharacter(
        &mut self,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetUnderlineSpecialCharacter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetUnderlineSpecialCharacter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (generationSettings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertNewLine(
        &mut self,
        i: i32,
        baseScale: f32,
        currentElementScale: f32,
        currentEmScale: f32,
        boldSpacingAdjustment: f32,
        characterSpacingAdjustment: f32,
        width: f32,
        lineGap: f32,
        isMaxVisibleDescenderSet: quest_hook::libil2cpp::ByRefMut<bool>,
        maxVisibleDescender: quest_hook::libil2cpp::ByRefMut<f32>,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                12usize,
            >("InsertNewLine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "InsertNewLine", 12usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        i,
                        baseScale,
                        currentElementScale,
                        currentEmScale,
                        boldSpacingAdjustment,
                        characterSpacingAdjustment,
                        width,
                        lineGap,
                        isMaxVisibleDescenderSet,
                        maxVisibleDescender,
                        generationSettings,
                        textInfo,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PopulateTextBackingArray_Il2CppString0(
        &mut self,
        sourceText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PopulateTextBackingArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "PopulateTextBackingArray",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sourceText))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateTextBackingArray_i32_i32_1(
        &mut self,
        sourceText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("PopulateTextBackingArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "PopulateTextBackingArray",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sourceText, start, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateTextProcessingArray(
        &mut self,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PopulateTextProcessingArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "PopulateTextProcessingArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (generationSettings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Prepare(
        &mut self,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Prepare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "Prepare", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (generationSettings, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RestoreWordWrappingState(
        &mut self,
        state: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::Text::WordWrapState,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::TextCore::Text::WordWrapState,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                i32,
                2usize,
            >("RestoreWordWrappingState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "RestoreWordWrappingState",
                    2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (state, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveGlyphVertexInfo(
        &mut self,
        padding: f32,
        stylePadding: f32,
        vertexColor: crate::UnityEngine::Color32,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    f32,
                    crate::UnityEngine::Color32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("SaveGlyphVertexInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "SaveGlyphVertexInfo",
                    5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (padding, stylePadding, vertexColor, generationSettings, textInfo),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveSpriteVertexInfo(
        &mut self,
        vertexColor: crate::UnityEngine::Color32,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Color32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SaveSpriteVertexInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "SaveSpriteVertexInfo",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (vertexColor, generationSettings, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveWordWrappingState(
        &mut self,
        state: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::Text::WordWrapState,
        >,
        index: i32,
        count: i32,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::TextCore::Text::WordWrapState,
                    >,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SaveWordWrappingState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "SaveWordWrappingState",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, index, count, textInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetArraySizes(
        &mut self,
        textProcessingArray: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::Text::TextProcessingElement,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::TextCore::Text::TextProcessingElement,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                i32,
                3usize,
            >("SetArraySizes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "SetArraySizes", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (textProcessingArray, generationSettings, textInfo),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateHtmlTag(
        &mut self,
        chars: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::Text::TextProcessingElement,
            >,
        >,
        startIndex: i32,
        endIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::TextCore::Text::TextProcessingElement,
                        >,
                    >,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                ),
                bool,
                5usize,
            >("ValidateHtmlTag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "ValidateHtmlTag", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (chars, startIndex, endIndex, generationSettings, textInfo),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isTextTruncated() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_isTextTruncated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "get_isTextTruncated",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+MissingCharacterEventCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct TextGenerator_MissingCharacterEventCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+MissingCharacterEventCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::TextGenerator_MissingCharacterEventCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextGenerator/MissingCharacterEventCallback";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+MissingCharacterEventCallback")]
impl std::ops::Deref
for crate::UnityEngine::TextCore::Text::TextGenerator_MissingCharacterEventCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+MissingCharacterEventCallback")]
impl std::ops::DerefMut
for crate::UnityEngine::TextCore::Text::TextGenerator_MissingCharacterEventCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+MissingCharacterEventCallback")]
impl crate::UnityEngine::TextCore::Text::TextGenerator_MissingCharacterEventCallback {
    pub fn Invoke(
        &mut self,
        unicode: u32,
        stringIndex: i32,
        text: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator_MissingCharacterEventCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::FontAsset,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::TextCore::Text::TextGenerator_MissingCharacterEventCallback
                    as quest_hook::libil2cpp::Type > ::class(), "Invoke", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (unicode, stringIndex, text, fontAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator_MissingCharacterEventCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::TextCore::Text::TextGenerator_MissingCharacterEventCallback
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+MissingCharacterEventCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextGenerator_MissingCharacterEventCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+SpecialCharacter")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextGenerator_SpecialCharacter {
    pub character: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::Character,
    >,
    pub fontAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::FontAsset,
    >,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub materialIndex: i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+SpecialCharacter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextGenerator/SpecialCharacter";
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
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+SpecialCharacter")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+SpecialCharacter")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter {
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
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+SpecialCharacter")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter {
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
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+SpecialCharacter")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter {
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
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+SpecialCharacter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerator+SpecialCharacter")]
impl crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter {
    pub fn _ctor(
        &mut self,
        character: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::Character,
        >,
        materialIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::Character,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::TextGenerator_SpecialCharacter
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (character, materialIndex))?
        };
        Ok(__cordl_ret.into())
    }
}
