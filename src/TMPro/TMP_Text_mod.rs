#[cfg(feature = "TMPro+TMP_Text")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_Text {
    __cordl_parent: crate::UnityEngine::UI::MaskableGraphic,
    pub m_text: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_IsTextBackingStringDirty: bool,
    pub m_TextPreprocessor: *mut crate::TMPro::ITextPreprocessor,
    pub m_isRightToLeft: bool,
    pub m_fontAsset: *mut crate::TMPro::TMP_FontAsset,
    pub m_currentFontAsset: *mut crate::TMPro::TMP_FontAsset,
    pub m_isSDFShader: bool,
    pub m_sharedMaterial: *mut crate::UnityEngine::Material,
    pub m_currentMaterial: *mut crate::UnityEngine::Material,
    pub m_currentMaterialIndex: i32,
    pub m_fontSharedMaterials: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Material,
    >,
    pub m_fontMaterial: *mut crate::UnityEngine::Material,
    pub m_fontMaterials: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Material,
    >,
    pub m_isMaterialDirty: bool,
    pub m_fontColor32: crate::UnityEngine::Color32,
    pub m_fontColor: crate::UnityEngine::Color,
    pub m_underlineColor: crate::UnityEngine::Color32,
    pub m_strikethroughColor: crate::UnityEngine::Color32,
    pub m_enableVertexGradient: bool,
    pub m_colorMode: crate::TMPro::ColorMode,
    pub m_fontColorGradient: crate::TMPro::VertexGradient,
    pub m_fontColorGradientPreset: *mut crate::TMPro::TMP_ColorGradient,
    pub m_spriteAsset: *mut crate::TMPro::TMP_SpriteAsset,
    pub m_tintAllSprites: bool,
    pub m_tintSprite: bool,
    pub m_spriteColor: crate::UnityEngine::Color32,
    pub m_StyleSheet: *mut crate::TMPro::TMP_StyleSheet,
    pub m_TextStyle: *mut crate::TMPro::TMP_Style,
    pub m_TextStyleHashCode: i32,
    pub m_overrideHtmlColors: bool,
    pub m_faceColor: crate::UnityEngine::Color32,
    pub m_outlineColor: crate::UnityEngine::Color32,
    pub m_outlineWidth: f32,
    pub m_fontSize: f32,
    pub m_currentFontSize: f32,
    pub m_fontSizeBase: f32,
    pub m_sizeStack: crate::TMPro::TMP_TextProcessingStack_1<f32>,
    pub m_fontWeight: crate::TMPro::FontWeight,
    pub m_FontWeightInternal: crate::TMPro::FontWeight,
    pub m_FontWeightStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::TMPro::FontWeight,
    >,
    pub m_enableAutoSizing: bool,
    pub m_maxFontSize: f32,
    pub m_minFontSize: f32,
    pub m_AutoSizeIterationCount: i32,
    pub m_AutoSizeMaxIterationCount: i32,
    pub m_IsAutoSizePointSizeSet: bool,
    pub m_fontSizeMin: f32,
    pub m_fontSizeMax: f32,
    pub m_fontStyle: crate::TMPro::FontStyles,
    pub m_FontStyleInternal: crate::TMPro::FontStyles,
    pub m_fontStyleStack: crate::TMPro::TMP_FontStyleStack,
    pub m_isUsingBold: bool,
    pub m_HorizontalAlignment: crate::TMPro::HorizontalAlignmentOptions,
    pub m_VerticalAlignment: crate::TMPro::VerticalAlignmentOptions,
    pub m_textAlignment: crate::TMPro::TextAlignmentOptions,
    pub m_lineJustification: crate::TMPro::HorizontalAlignmentOptions,
    pub m_lineJustificationStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::TMPro::HorizontalAlignmentOptions,
    >,
    pub m_textContainerLocalCorners: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
    pub m_characterSpacing: f32,
    pub m_cSpacing: f32,
    pub m_monoSpacing: f32,
    pub m_wordSpacing: f32,
    pub m_lineSpacing: f32,
    pub m_lineSpacingDelta: f32,
    pub m_lineHeight: f32,
    pub m_IsDrivenLineSpacing: bool,
    pub m_lineSpacingMax: f32,
    pub m_paragraphSpacing: f32,
    pub m_charWidthMaxAdj: f32,
    pub m_charWidthAdjDelta: f32,
    pub m_enableWordWrapping: bool,
    pub m_isCharacterWrappingEnabled: bool,
    pub m_isNonBreakingSpace: bool,
    pub m_isIgnoringAlignment: bool,
    pub m_wordWrappingRatios: f32,
    pub m_overflowMode: crate::TMPro::TextOverflowModes,
    pub m_firstOverflowCharacterIndex: i32,
    pub m_linkedTextComponent: *mut crate::TMPro::TMP_Text,
    pub parentLinkedComponent: *mut crate::TMPro::TMP_Text,
    pub m_isTextTruncated: bool,
    pub m_enableKerning: bool,
    pub m_GlyphHorizontalAdvanceAdjustment: f32,
    pub m_enableExtraPadding: bool,
    pub checkPaddingRequired: bool,
    pub m_isRichText: bool,
    pub m_parseCtrlCharacters: bool,
    pub m_isOverlay: bool,
    pub m_isOrthographic: bool,
    pub m_isCullingEnabled: bool,
    pub m_isMaskingEnabled: bool,
    pub isMaskUpdateRequired: bool,
    pub m_ignoreCulling: bool,
    pub m_horizontalMapping: crate::TMPro::TextureMappingOptions,
    pub m_verticalMapping: crate::TMPro::TextureMappingOptions,
    pub m_uvLineOffset: f32,
    pub m_renderMode: crate::TMPro::TextRenderFlags,
    pub m_geometrySortingOrder: crate::TMPro::VertexSortingOrder,
    pub m_IsTextObjectScaleStatic: bool,
    pub m_VertexBufferAutoSizeReduction: bool,
    pub m_firstVisibleCharacter: i32,
    pub m_maxVisibleCharacters: i32,
    pub m_maxVisibleWords: i32,
    pub m_maxVisibleLines: i32,
    pub m_useMaxVisibleDescender: bool,
    pub m_pageToDisplay: i32,
    pub m_isNewPage: bool,
    pub m_margin: crate::UnityEngine::Vector4,
    pub m_marginLeft: f32,
    pub m_marginRight: f32,
    pub m_marginWidth: f32,
    pub m_marginHeight: f32,
    pub m_width: f32,
    pub m_textInfo: *mut crate::TMPro::TMP_TextInfo,
    pub m_havePropertiesChanged: bool,
    pub m_isUsingLegacyAnimationComponent: bool,
    pub m_transform: *mut crate::UnityEngine::Transform,
    pub m_rectTransform: *mut crate::UnityEngine::RectTransform,
    pub m_PreviousRectTransformSize: crate::UnityEngine::Vector2,
    pub m_PreviousPivotPosition: crate::UnityEngine::Vector2,
    pub _autoSizeTextContainer_k__BackingField: bool,
    pub m_autoSizeTextContainer: bool,
    pub m_mesh: *mut crate::UnityEngine::Mesh,
    pub m_isVolumetricText: bool,
    pub OnPreRenderText: *mut crate::System::Action_1<*mut crate::TMPro::TMP_TextInfo>,
    pub m_spriteAnimator: *mut crate::TMPro::TMP_SpriteAnimator,
    pub m_flexibleHeight: f32,
    pub m_flexibleWidth: f32,
    pub m_minWidth: f32,
    pub m_minHeight: f32,
    pub m_maxWidth: f32,
    pub m_maxHeight: f32,
    pub m_LayoutElement: *mut crate::UnityEngine::UI::LayoutElement,
    pub m_preferredWidth: f32,
    pub m_renderedWidth: f32,
    pub m_isPreferredWidthDirty: bool,
    pub m_preferredHeight: f32,
    pub m_renderedHeight: f32,
    pub m_isPreferredHeightDirty: bool,
    pub m_isCalculatingPreferredValues: bool,
    pub m_layoutPriority: i32,
    pub m_isLayoutDirty: bool,
    pub m_isAwake: bool,
    pub m_isWaitingOnResourceLoad: bool,
    pub m_inputSource: crate::TMPro::TMP_Text_TextInputSources,
    pub m_fontScaleMultiplier: f32,
    pub tag_LineIndent: f32,
    pub tag_Indent: f32,
    pub m_indentStack: crate::TMPro::TMP_TextProcessingStack_1<f32>,
    pub tag_NoParsing: bool,
    pub m_isParsingText: bool,
    pub m_FXMatrix: crate::UnityEngine::Matrix4x4,
    pub m_isFXMatrixSet: bool,
    pub m_TextProcessingArray: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::TMPro::TMP_Text_UnicodeChar,
    >,
    pub m_InternalTextProcessingArraySize: i32,
    pub m_internalCharacterInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::TMPro::TMP_CharacterInfo,
    >,
    pub m_totalCharacterCount: i32,
    pub m_characterCount: i32,
    pub m_firstCharacterOfLine: i32,
    pub m_firstVisibleCharacterOfLine: i32,
    pub m_lastCharacterOfLine: i32,
    pub m_lastVisibleCharacterOfLine: i32,
    pub m_lineNumber: i32,
    pub m_lineVisibleCharacterCount: i32,
    pub m_pageNumber: i32,
    pub m_PageAscender: f32,
    pub m_maxTextAscender: f32,
    pub m_maxCapHeight: f32,
    pub m_ElementAscender: f32,
    pub m_ElementDescender: f32,
    pub m_maxLineAscender: f32,
    pub m_maxLineDescender: f32,
    pub m_startOfLineAscender: f32,
    pub m_startOfLineDescender: f32,
    pub m_lineOffset: f32,
    pub m_meshExtents: crate::TMPro::Extents,
    pub m_htmlColor: crate::UnityEngine::Color32,
    pub m_colorStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub m_underlineColorStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub m_strikethroughColorStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub m_HighlightStateStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::TMPro::HighlightState,
    >,
    pub m_colorGradientPreset: *mut crate::TMPro::TMP_ColorGradient,
    pub m_colorGradientStack: crate::TMPro::TMP_TextProcessingStack_1<
        *mut crate::TMPro::TMP_ColorGradient,
    >,
    pub m_colorGradientPresetIsTinted: bool,
    pub m_tabSpacing: f32,
    pub m_spacing: f32,
    pub m_TextStyleStacks: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::TMPro::TMP_TextProcessingStack_1<i32>,
    >,
    pub m_TextStyleStackDepth: i32,
    pub m_ItalicAngleStack: crate::TMPro::TMP_TextProcessingStack_1<i32>,
    pub m_ItalicAngle: i32,
    pub m_actionStack: crate::TMPro::TMP_TextProcessingStack_1<i32>,
    pub m_padding: f32,
    pub m_baselineOffset: f32,
    pub m_baselineOffsetStack: crate::TMPro::TMP_TextProcessingStack_1<f32>,
    pub m_xAdvance: f32,
    pub m_textElementType: crate::TMPro::TMP_TextElementType,
    pub m_cached_TextElement: *mut crate::TMPro::TMP_TextElement,
    pub m_Ellipsis: crate::TMPro::TMP_Text_SpecialCharacter,
    pub m_Underline: crate::TMPro::TMP_Text_SpecialCharacter,
    pub m_defaultSpriteAsset: *mut crate::TMPro::TMP_SpriteAsset,
    pub m_currentSpriteAsset: *mut crate::TMPro::TMP_SpriteAsset,
    pub m_spriteCount: i32,
    pub m_spriteIndex: i32,
    pub m_spriteAnimationID: i32,
    pub m_ignoreActiveState: bool,
    pub m_TextBackingArray: crate::TMPro::TMP_Text_TextBackingContainer,
    pub k_Power: *mut quest_hook::libil2cpp::Il2CppArray<crate::System::Decimal>,
}
#[cfg(feature = "TMPro+TMP_Text")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Text => "TMPro"."TMP_Text"
);
#[cfg(feature = "TMPro+TMP_Text")]
impl std::ops::Deref for crate::TMPro::TMP_Text {
    type Target = crate::UnityEngine::UI::MaskableGraphic;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Text")]
impl std::ops::DerefMut for crate::TMPro::TMP_Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Text")]
impl crate::TMPro::TMP_Text {
    #[cfg(feature = "TMPro+TMP_Text+CharacterSubstitution")]
    pub type CharacterSubstitution = crate::TMPro::TMP_Text_CharacterSubstitution;
    #[cfg(feature = "TMPro+TMP_Text+SpecialCharacter")]
    pub type SpecialCharacter = crate::TMPro::TMP_Text_SpecialCharacter;
    #[cfg(feature = "TMPro+TMP_Text+TextBackingContainer")]
    pub type TextBackingContainer = crate::TMPro::TMP_Text_TextBackingContainer;
    #[cfg(feature = "TMPro+TMP_Text+TextInputSources")]
    pub type TextInputSources = crate::TMPro::TMP_Text_TextInputSources;
    #[cfg(feature = "TMPro+TMP_Text+UnicodeChar")]
    pub type UnicodeChar = crate::TMPro::TMP_Text_UnicodeChar;
    #[cfg(feature = "TMPro+TMP_Text+__c")]
    pub type __c = crate::TMPro::TMP_Text___c;
    pub fn AddFloatToInternalTextBackingArray(
        &mut self,
        value: f32,
        padding: i32,
        precision: i32,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddFloatToInternalTextBackingArray",
                (value, padding, precision, writeIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddIntegerToInternalTextBackingArray(
        &mut self,
        number: f64,
        padding: i32,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddIntegerToInternalTextBackingArray",
                (number, padding, writeIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AdjustLineOffset(
        &mut self,
        startIndex: i32,
        endIndex: i32,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AdjustLineOffset", (startIndex, endIndex, offset))?;
        Ok(__cordl_ret)
    }
    pub fn CalculatePreferredValues(
        &mut self,
        fontSize: quest_hook::libil2cpp::ByRefMut<f32>,
        marginSize: crate::UnityEngine::Vector2,
        isTextAutoSizingEnabled: bool,
        isWordWrappingEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke(
                "CalculatePreferredValues",
                (fontSize, marginSize, isTextAutoSizingEnabled, isWordWrappingEnabled),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ClearMesh_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearMesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearMesh__cordl_bool1(
        &mut self,
        uploadGeometry: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearMesh", (uploadGeometry))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeMarginSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeMarginSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConvertToFloat_ByRefMut1(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        startIndex: i32,
        length: i32,
        lastIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ConvertToFloat", (chars, startIndex, length, lastIndex))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertToFloat_Il2CppArray_i32_i32_0(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ConvertToFloat", (chars, startIndex, length))?;
        Ok(__cordl_ret)
    }
    pub fn CreateMaterialInstance(
        &mut self,
        source: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("CreateMaterialInstance", (source))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeAlpha(
        &mut self,
        alpha: f32,
        duration: f32,
        ignoreTimeScale: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossFadeAlpha", (alpha, duration, ignoreTimeScale))?;
        Ok(__cordl_ret)
    }
    pub fn CrossFadeColor(
        &mut self,
        targetColor: crate::UnityEngine::Color,
        duration: f32,
        ignoreTimeScale: bool,
        useAlpha: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossFadeColor",
                (targetColor, duration, ignoreTimeScale, useAlpha),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DestroySubMeshObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroySubMeshObjects", ())?;
        Ok(__cordl_ret)
    }
    pub fn DrawTextHighlight(
        &mut self,
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
        highlightColor: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawTextHighlight", (start, end, index, highlightColor))?;
        Ok(__cordl_ret)
    }
    pub fn DrawUnderlineMesh(
        &mut self,
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
        startScale: f32,
        endScale: f32,
        maxScale: f32,
        sdfScale: f32,
        underlineColor: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DrawUnderlineMesh",
                (
                    start,
                    end,
                    index,
                    startScale,
                    endScale,
                    maxScale,
                    sdfScale,
                    underlineColor,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn FillCharacterVertexBuffers__cordl_bool1(
        &mut self,
        i: i32,
        index_X4: i32,
        isVolumetric: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillCharacterVertexBuffers", (i, index_X4, isVolumetric))?;
        Ok(__cordl_ret)
    }
    pub fn FillCharacterVertexBuffers_i32_i32_0(
        &mut self,
        i: i32,
        index_X4: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillCharacterVertexBuffers", (i, index_X4))?;
        Ok(__cordl_ret)
    }
    pub fn FillSpriteVertexBuffers(
        &mut self,
        i: i32,
        index_X4: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillSpriteVertexBuffers", (i, index_X4))?;
        Ok(__cordl_ret)
    }
    pub fn ForceMeshUpdate(
        &mut self,
        ignoreActiveState: bool,
        forceTextReparsing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceMeshUpdate", (ignoreActiveState, forceTextReparsing))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributeParameters(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        startIndex: i32,
        length: i32,
        parameters: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetAttributeParameters", (chars, startIndex, length, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn GetCanvasSpaceClippingRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("GetCanvasSpaceClippingRect", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCompoundBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("GetCompoundBounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEllipsisSpecialCharacter(
        &mut self,
        fontAsset: *mut crate::TMPro::TMP_FontAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetEllipsisSpecialCharacter", (fontAsset))?;
        Ok(__cordl_ret)
    }
    pub fn GetFontAssetForWeight(
        &mut self,
        fontWeight: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_FontAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_FontAsset = __cordl_object
            .invoke("GetFontAssetForWeight", (fontWeight))?;
        Ok(__cordl_ret)
    }
    pub fn GetMarkupTagHashCode_Il2CppArray0(
        &mut self,
        tagDefinition: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        readIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetMarkupTagHashCode", (tagDefinition, readIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetMarkupTagHashCode_TMP_Text_TextBackingContainer1(
        &mut self,
        tagDefinition: crate::TMPro::TMP_Text_TextBackingContainer,
        readIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetMarkupTagHashCode", (tagDefinition, readIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetMaterial(
        &mut self,
        mat: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("GetMaterial", (mat))?;
        Ok(__cordl_ret)
    }
    pub fn GetMaterials(
        &mut self,
        mats: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Material,
        > = __cordl_object.invoke("GetMaterials", (mats))?;
        Ok(__cordl_ret)
    }
    pub fn GetPaddingForMaterial_0(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPaddingForMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPaddingForMaterial_Material1(
        &mut self,
        mat: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPaddingForMaterial", (mat))?;
        Ok(__cordl_ret)
    }
    pub fn GetParsedText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetParsedText", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredHeight_0(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPreferredHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredHeight_Vector2_1(
        &mut self,
        margin: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPreferredHeight", (margin))?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredValues_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetPreferredValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredValues_Il2CppString2(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetPreferredValues", (text))?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredValues_Il2CppString_f32_f32_3(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetPreferredValues", (text, width, height))?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredValues_f32_f32_1(
        &mut self,
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetPreferredValues", (width, height))?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredWidth_0(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPreferredWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredWidth_Vector2_1(
        &mut self,
        margin: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPreferredWidth", (margin))?;
        Ok(__cordl_ret)
    }
    pub fn GetRenderedHeight_0(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetRenderedHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRenderedHeight__cordl_bool1(
        &mut self,
        onlyVisibleCharacters: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetRenderedHeight", (onlyVisibleCharacters))?;
        Ok(__cordl_ret)
    }
    pub fn GetRenderedValues_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetRenderedValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRenderedValues__cordl_bool1(
        &mut self,
        onlyVisibleCharacters: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetRenderedValues", (onlyVisibleCharacters))?;
        Ok(__cordl_ret)
    }
    pub fn GetRenderedWidth_0(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetRenderedWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRenderedWidth__cordl_bool1(
        &mut self,
        onlyVisibleCharacters: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetRenderedWidth", (onlyVisibleCharacters))?;
        Ok(__cordl_ret)
    }
    pub fn GetSharedMaterials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Material,
        > = __cordl_object.invoke("GetSharedMaterials", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSpecialCharacters(
        &mut self,
        fontAsset: *mut crate::TMPro::TMP_FontAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetSpecialCharacters", (fontAsset))?;
        Ok(__cordl_ret)
    }
    pub fn GetStyle(
        &mut self,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_Style> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_Style = __cordl_object
            .invoke("GetStyle", (hashCode))?;
        Ok(__cordl_ret)
    }
    pub fn GetStyleHashCode_ByRefMut_i32_ByRefMut0(
        &mut self,
        text: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        index: i32,
        closeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetStyleHashCode", (text, index, closeIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetStyleHashCode_ByRefMut_i32_ByRefMut1(
        &mut self,
        text: quest_hook::libil2cpp::ByRefMut<
            crate::TMPro::TMP_Text_TextBackingContainer,
        >,
        index: i32,
        closeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetStyleHashCode", (text, index, closeIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextBounds_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("GetTextBounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTextBounds__cordl_bool1(
        &mut self,
        onlyVisibleCharacters: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("GetTextBounds", (onlyVisibleCharacters))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextContainerLocalCorners(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector3,
        > = __cordl_object.invoke("GetTextContainerLocalCorners", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTextElement(
        &mut self,
        unicode: u32,
        fontAsset: *mut crate::TMPro::TMP_FontAsset,
        fontStyle: crate::TMPro::FontStyles,
        fontWeight: crate::TMPro::FontWeight,
        isUsingAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_TextElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_TextElement = __cordl_object
            .invoke(
                "GetTextElement",
                (unicode, fontAsset, fontStyle, fontWeight, isUsingAlternativeTypeface),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetTextInfo(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_TextInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_TextInfo = __cordl_object
            .invoke("GetTextInfo", (text))?;
        Ok(__cordl_ret)
    }
    pub fn GetUTF16_Il2CppArray1(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUTF16", (text, i))?;
        Ok(__cordl_ret)
    }
    pub fn GetUTF16_Il2CppArray2(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUTF16", (text, i))?;
        Ok(__cordl_ret)
    }
    pub fn GetUTF16_Il2CppString0(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUTF16", (text, i))?;
        Ok(__cordl_ret)
    }
    pub fn GetUTF16_StringBuilder3(
        &mut self,
        text: *mut crate::System::Text::StringBuilder,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUTF16", (text, i))?;
        Ok(__cordl_ret)
    }
    pub fn GetUTF16_TMP_Text_TextBackingContainer4(
        &mut self,
        text: crate::TMPro::TMP_Text_TextBackingContainer,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUTF16", (text, i))?;
        Ok(__cordl_ret)
    }
    pub fn GetUTF32_Il2CppArray1(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUTF32", (text, i))?;
        Ok(__cordl_ret)
    }
    pub fn GetUTF32_Il2CppArray2(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUTF32", (text, i))?;
        Ok(__cordl_ret)
    }
    pub fn GetUTF32_Il2CppString0(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUTF32", (text, i))?;
        Ok(__cordl_ret)
    }
    pub fn GetUTF32_StringBuilder3(
        &mut self,
        text: *mut crate::System::Text::StringBuilder,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUTF32", (text, i))?;
        Ok(__cordl_ret)
    }
    pub fn GetUTF32_TMP_Text_TextBackingContainer4(
        &mut self,
        text: crate::TMPro::TMP_Text_TextBackingContainer,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUTF32", (text, i))?;
        Ok(__cordl_ret)
    }
    pub fn GetUnderlineSpecialCharacter(
        &mut self,
        fontAsset: *mut crate::TMPro::TMP_FontAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetUnderlineSpecialCharacter", (fontAsset))?;
        Ok(__cordl_ret)
    }
    pub fn HexCharsToColor_Il2CppArray_i32_0(
        &mut self,
        hexChars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        tagCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color32 = __cordl_object
            .invoke("HexCharsToColor", (hexChars, tagCount))?;
        Ok(__cordl_ret)
    }
    pub fn HexCharsToColor_i32_1(
        &mut self,
        hexChars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color32 = __cordl_object
            .invoke("HexCharsToColor", (hexChars, startIndex, length))?;
        Ok(__cordl_ret)
    }
    pub fn HexToInt(&mut self, hex: char) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("HexToInt", (hex))?;
        Ok(__cordl_ret)
    }
    pub fn InsertClosingStyleTag(
        &mut self,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_Text_UnicodeChar>,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertClosingStyleTag", (charBuffer, writeIndex))?;
        Ok(__cordl_ret)
    }
    pub fn InsertNewLine(
        &mut self,
        i: i32,
        baseScale: f32,
        currentElementScale: f32,
        currentEmScale: f32,
        glyphAdjustment: f32,
        boldSpacingAdjustment: f32,
        characterSpacingAdjustment: f32,
        width: f32,
        lineGap: f32,
        isMaxVisibleDescenderSet: quest_hook::libil2cpp::ByRefMut<bool>,
        maxVisibleDescender: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InsertNewLine",
                (
                    i,
                    baseScale,
                    currentElementScale,
                    currentEmScale,
                    glyphAdjustment,
                    boldSpacingAdjustment,
                    characterSpacingAdjustment,
                    width,
                    lineGap,
                    isMaxVisibleDescenderSet,
                    maxVisibleDescender,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InsertOpeningStyleTag(
        &mut self,
        style: *mut crate::TMPro::TMP_Style,
        srcIndex: i32,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_Text_UnicodeChar>,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InsertOpeningStyleTag", (style, srcIndex, charBuffer, writeIndex))?;
        Ok(__cordl_ret)
    }
    pub fn InternalCrossFadeAlpha(
        &mut self,
        alpha: f32,
        duration: f32,
        ignoreTimeScale: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalCrossFadeAlpha", (alpha, duration, ignoreTimeScale))?;
        Ok(__cordl_ret)
    }
    pub fn InternalCrossFadeColor(
        &mut self,
        targetColor: crate::UnityEngine::Color,
        duration: f32,
        ignoreTimeScale: bool,
        useAlpha: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InternalCrossFadeColor",
                (targetColor, duration, ignoreTimeScale, useAlpha),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InternalTextBackingArrayToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("InternalTextBackingArrayToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsSelfOrLinkedAncestor(
        &mut self,
        targetTextComponent: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSelfOrLinkedAncestor", (targetTextComponent))?;
        Ok(__cordl_ret)
    }
    pub fn LoadDefaultSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadDefaultSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadFontAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadFontAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PackUV_f32_0(
        &mut self,
        x: f32,
        y: f32,
        scale: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("PackUV", (x, y, scale))?;
        Ok(__cordl_ret)
    }
    pub fn PackUV_f32_f32_1(
        &mut self,
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("PackUV", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn ParseInputText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseInputText", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopulateTextBackingArray_Il2CppArray_i32_i32_3(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateTextBackingArray", (sourceText, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateTextBackingArray_Il2CppString0(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateTextBackingArray", (sourceText))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateTextBackingArray_Il2CppString_i32_i32_1(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateTextBackingArray", (sourceText, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateTextBackingArray_StringBuilder_i32_i32_2(
        &mut self,
        sourceText: *mut crate::System::Text::StringBuilder,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateTextBackingArray", (sourceText, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateTextProcessingArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateTextProcessingArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseLinkedTextComponent(
        &mut self,
        targetTextComponent: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseLinkedTextComponent", (targetTextComponent))?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceClosingStyleTag_ByRefMut_i32_ByRefMut_ByRefMut0(
        &mut self,
        sourceText: quest_hook::libil2cpp::ByRefMut<
            crate::TMPro::TMP_Text_TextBackingContainer,
        >,
        srcIndex: i32,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_Text_UnicodeChar>,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReplaceClosingStyleTag",
                (sourceText, srcIndex, charBuffer, writeIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceClosingStyleTag_ByRefMut_i32_ByRefMut_ByRefMut1(
        &mut self,
        sourceText: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        srcIndex: i32,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_Text_UnicodeChar>,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReplaceClosingStyleTag",
                (sourceText, srcIndex, charBuffer, writeIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceOpeningStyleTag_ByRefMut_i32_ByRefMut_ByRefMut_ByRefMut0(
        &mut self,
        sourceText: quest_hook::libil2cpp::ByRefMut<
            crate::TMPro::TMP_Text_TextBackingContainer,
        >,
        srcIndex: i32,
        srcOffset: quest_hook::libil2cpp::ByRefMut<i32>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_Text_UnicodeChar>,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ReplaceOpeningStyleTag",
                (sourceText, srcIndex, srcOffset, charBuffer, writeIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceOpeningStyleTag_ByRefMut_i32_ByRefMut_ByRefMut_ByRefMut1(
        &mut self,
        sourceText: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        srcIndex: i32,
        srcOffset: quest_hook::libil2cpp::ByRefMut<i32>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_Text_UnicodeChar>,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ReplaceOpeningStyleTag",
                (sourceText, srcIndex, srcOffset, charBuffer, writeIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceTagWithCharacter(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        insertionIndex: i32,
        tagLength: i32,
        c: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplaceTagWithCharacter", (chars, insertionIndex, tagLength, c))?;
        Ok(__cordl_ret)
    }
    pub fn ResizeInternalArray_ByRefMut0<T>(
        &mut self,
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResizeInternalArray", (array))?;
        Ok(__cordl_ret)
    }
    pub fn ResizeInternalArray_i32_1<T>(
        &mut self,
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<T>,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResizeInternalArray", (array, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn ResizeLineExtents(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResizeLineExtents", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn RestoreWordWrappingState(
        &mut self,
        state: quest_hook::libil2cpp::ByRefMut<crate::TMPro::WordWrapState>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("RestoreWordWrappingState", (state))?;
        Ok(__cordl_ret)
    }
    pub fn SaveGlyphVertexInfo(
        &mut self,
        padding: f32,
        style_padding: f32,
        vertexColor: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveGlyphVertexInfo", (padding, style_padding, vertexColor))?;
        Ok(__cordl_ret)
    }
    pub fn SaveSpriteVertexInfo(
        &mut self,
        vertexColor: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveSpriteVertexInfo", (vertexColor))?;
        Ok(__cordl_ret)
    }
    pub fn SaveWordWrappingState(
        &mut self,
        state: quest_hook::libil2cpp::ByRefMut<crate::TMPro::WordWrapState>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveWordWrappingState", (state, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn SetActiveSubMeshes(
        &mut self,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActiveSubMeshes", (state))?;
        Ok(__cordl_ret)
    }
    pub fn SetArraySizes(
        &mut self,
        unicodeChars: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::TMPro::TMP_Text_UnicodeChar,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("SetArraySizes", (unicodeChars))?;
        Ok(__cordl_ret)
    }
    pub fn SetCharArray_Il2CppArray0(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCharArray", (sourceText))?;
        Ok(__cordl_ret)
    }
    pub fn SetCharArray_i32_i32_1(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCharArray", (sourceText, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn SetCulling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCulling", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetFaceColor(
        &mut self,
        color: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFaceColor", (color))?;
        Ok(__cordl_ret)
    }
    pub fn SetFontBaseMaterial(
        &mut self,
        mat: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFontBaseMaterial", (mat))?;
        Ok(__cordl_ret)
    }
    pub fn SetOutlineColor(
        &mut self,
        color: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOutlineColor", (color))?;
        Ok(__cordl_ret)
    }
    pub fn SetOutlineThickness(
        &mut self,
        thickness: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOutlineThickness", (thickness))?;
        Ok(__cordl_ret)
    }
    pub fn SetShaderDepth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetShaderDepth", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetSharedMaterial(
        &mut self,
        mat: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSharedMaterial", (mat))?;
        Ok(__cordl_ret)
    }
    pub fn SetSharedMaterials(
        &mut self,
        materials: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Material,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSharedMaterials", (materials))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextInternal(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextInternal", (sourceText))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextSortingOrder_Il2CppArray1(
        &mut self,
        order: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextSortingOrder", (order))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextSortingOrder_VertexSortingOrder0(
        &mut self,
        order: crate::TMPro::VertexSortingOrder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextSortingOrder", (order))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppArray11(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppArray_i32_i32_12(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppString__cordl_bool0(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
        syncTextInputBox: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText, syncTextInputBox))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppString_f32_1(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText, arg0))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppString_f32_f32_2(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: f32,
        arg1: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText, arg0, arg1))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppString_f32_f32_f32_3(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: f32,
        arg1: f32,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText, arg0, arg1, arg2))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppString_f32_f32_f32_f32_4(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: f32,
        arg1: f32,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText, arg0, arg1, arg2, arg3))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppString_f32_f32_f32_f32_f32_5(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: f32,
        arg1: f32,
        arg2: f32,
        arg3: f32,
        arg4: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText, arg0, arg1, arg2, arg3, arg4))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppString_f32_f32_f32_f32_f32_f32_6(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: f32,
        arg1: f32,
        arg2: f32,
        arg3: f32,
        arg4: f32,
        arg5: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText, arg0, arg1, arg2, arg3, arg4, arg5))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppString_f32_f32_f32_f32_f32_f32_f32_7(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: f32,
        arg1: f32,
        arg2: f32,
        arg3: f32,
        arg4: f32,
        arg5: f32,
        arg6: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText, arg0, arg1, arg2, arg3, arg4, arg5, arg6))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_Il2CppString_f32_f32_f32_f32_f32_f32_f32_f32_8(
        &mut self,
        sourceText: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: f32,
        arg1: f32,
        arg2: f32,
        arg3: f32,
        arg4: f32,
        arg5: f32,
        arg6: f32,
        arg7: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetText",
                (sourceText, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetText_StringBuilder9(
        &mut self,
        sourceText: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText))?;
        Ok(__cordl_ret)
    }
    pub fn SetText_StringBuilder_i32_i32_10(
        &mut self,
        sourceText: *mut crate::System::Text::StringBuilder,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (sourceText, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn SetVertexColorGradient(
        &mut self,
        gradient: *mut crate::TMPro::TMP_ColorGradient,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertexColorGradient", (gradient))?;
        Ok(__cordl_ret)
    }
    pub fn SetVertices(
        &mut self,
        vertices: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertices", (vertices))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCulling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCulling", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateGeometry(
        &mut self,
        mesh: *mut crate::UnityEngine::Mesh,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGeometry", (mesh, index))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMeshPadding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMeshPadding", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateVertexData_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVertexData", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateVertexData_TMP_VertexDataUpdateFlags0(
        &mut self,
        flags: crate::TMPro::TMP_VertexDataUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVertexData", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateHtmlTag(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::TMPro::TMP_Text_UnicodeChar,
        >,
        startIndex: i32,
        endIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateHtmlTag", (chars, startIndex, endIndex))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_OnPreRenderText(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::TMPro::TMP_TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_OnPreRenderText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_alignment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TextAlignmentOptions> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TextAlignmentOptions = __cordl_object
            .invoke("get_alignment", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_alpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_alpha", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_autoSizeTextContainer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_autoSizeTextContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("get_bounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characterSpacing(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_characterSpacing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characterWidthAdjustment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_characterWidthAdjustment", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorGradient(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::VertexGradient> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::VertexGradient = __cordl_object
            .invoke("get_colorGradient", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorGradientPreset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_ColorGradient> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_ColorGradient = __cordl_object
            .invoke("get_colorGradientPreset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enableAutoSizing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableAutoSizing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enableCulling(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableCulling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enableKerning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableKerning", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enableVertexGradient(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableVertexGradient", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enableWordWrapping(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableWordWrapping", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_extraPadding(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_extraPadding", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_faceColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color32 = __cordl_object
            .invoke("get_faceColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_firstOverflowCharacterIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_firstOverflowCharacterIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_firstVisibleCharacter(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_firstVisibleCharacter", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flexibleHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexibleHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flexibleWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexibleWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_font(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_FontAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_FontAsset = __cordl_object
            .invoke("get_font", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_fontMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontMaterials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Material,
        > = __cordl_object.invoke("get_fontMaterials", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontSharedMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_fontSharedMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontSharedMaterials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Material,
        > = __cordl_object.invoke("get_fontSharedMaterials", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fontSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontSizeMax(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fontSizeMax", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontSizeMin(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fontSizeMin", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontStyle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::FontStyles> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::FontStyles = __cordl_object
            .invoke("get_fontStyle", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontWeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::FontWeight> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::FontWeight = __cordl_object
            .invoke("get_fontWeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_geometrySortingOrder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::VertexSortingOrder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::VertexSortingOrder = __cordl_object
            .invoke("get_geometrySortingOrder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_havePropertiesChanged(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_havePropertiesChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalAlignment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::HorizontalAlignmentOptions> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::HorizontalAlignmentOptions = __cordl_object
            .invoke("get_horizontalAlignment", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalMapping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TextureMappingOptions> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TextureMappingOptions = __cordl_object
            .invoke("get_horizontalMapping", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ignoreVisibility(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ignoreVisibility", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isOrthographic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isOrthographic", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isOverlay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isOverlay", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isRightToLeftText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isRightToLeftText", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isTextObjectScaleStatic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isTextObjectScaleStatic", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isTextOverflowing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isTextOverflowing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isTextTruncated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isTextTruncated", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isUsingBold(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isUsingBold", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isUsingLegacyAnimationComponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isUsingLegacyAnimationComponent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isVolumetricText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isVolumetricText", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_layoutElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::LayoutElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::LayoutElement = __cordl_object
            .invoke("get_layoutElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_layoutPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layoutPriority", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lineSpacing(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lineSpacing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lineSpacingAdjustment(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lineSpacingAdjustment", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_linkedTextComponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_Text> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_Text = __cordl_object
            .invoke("get_linkedTextComponent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mappingUvLineOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_mappingUvLineOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_margin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("get_margin", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxVisibleCharacters(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxVisibleCharacters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxVisibleLines(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxVisibleLines", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxVisibleWords(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxVisibleWords", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Mesh> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Mesh = __cordl_object
            .invoke("get_mesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_outlineColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color32 = __cordl_object
            .invoke("get_outlineColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_outlineWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_outlineWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_overflowMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TextOverflowModes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TextOverflowModes = __cordl_object
            .invoke("get_overflowMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_overrideColorTags(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_overrideColorTags", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pageToDisplay(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_pageToDisplay", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_paragraphSpacing(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_paragraphSpacing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_parseCtrlCharacters(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_parseCtrlCharacters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pixelsPerUnit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pixelsPerUnit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_preferredHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_preferredHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_preferredWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_preferredWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectTransform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectTransform = __cordl_object
            .invoke("get_rectTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_renderMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TextRenderFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TextRenderFlags = __cordl_object
            .invoke("get_renderMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_renderedHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_renderedHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_renderedWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_renderedWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_richText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_richText", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_spriteAnimator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_SpriteAnimator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_SpriteAnimator = __cordl_object
            .invoke("get_spriteAnimator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_spriteAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_SpriteAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_SpriteAsset = __cordl_object
            .invoke("get_spriteAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_styleSheet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_StyleSheet> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_StyleSheet = __cordl_object
            .invoke("get_styleSheet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_text(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_text", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("get_textBounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_TextInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_TextInfo = __cordl_object
            .invoke("get_textInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textPreprocessor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::ITextPreprocessor> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::ITextPreprocessor = __cordl_object
            .invoke("get_textPreprocessor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textStyle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_Style> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_Style = __cordl_object
            .invoke("get_textStyle", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_tintAllSprites(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_tintAllSprites", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_transform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useMaxVisibleDescender(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useMaxVisibleDescender", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_vertexBufferAutoSizeReduction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_vertexBufferAutoSizeReduction", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalAlignment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::VerticalAlignmentOptions> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::VerticalAlignmentOptions = __cordl_object
            .invoke("get_verticalAlignment", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalMapping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TextureMappingOptions> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TextureMappingOptions = __cordl_object
            .invoke("get_verticalMapping", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_wordSpacing(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_wordSpacing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_wordWrappingRatios(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_wordWrappingRatios", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_OnPreRenderText(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::TMPro::TMP_TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_OnPreRenderText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_alignment(
        &mut self,
        value: crate::TMPro::TextAlignmentOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alignment", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_alpha(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alpha", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_autoSizeTextContainer(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_autoSizeTextContainer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_characterSpacing(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_characterSpacing", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_characterWidthAdjustment(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_characterWidthAdjustment", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_colorGradient(
        &mut self,
        value: crate::TMPro::VertexGradient,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorGradient", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_colorGradientPreset(
        &mut self,
        value: *mut crate::TMPro::TMP_ColorGradient,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorGradientPreset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_enableAutoSizing(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableAutoSizing", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_enableCulling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableCulling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_enableKerning(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableKerning", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_enableVertexGradient(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableVertexGradient", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_enableWordWrapping(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableWordWrapping", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_extraPadding(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_extraPadding", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_faceColor(
        &mut self,
        value: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_faceColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_firstVisibleCharacter(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_firstVisibleCharacter", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_font(
        &mut self,
        value: *mut crate::TMPro::TMP_FontAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_font", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontMaterial(
        &mut self,
        value: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontMaterial", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontMaterials(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontMaterials", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontSharedMaterial(
        &mut self,
        value: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontSharedMaterial", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontSharedMaterials(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontSharedMaterials", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontSize", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontSizeMax(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontSizeMax", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontSizeMin(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontSizeMin", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontStyle(
        &mut self,
        value: crate::TMPro::FontStyles,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontStyle", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontWeight(
        &mut self,
        value: crate::TMPro::FontWeight,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontWeight", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_geometrySortingOrder(
        &mut self,
        value: crate::TMPro::VertexSortingOrder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_geometrySortingOrder", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_havePropertiesChanged(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_havePropertiesChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalAlignment(
        &mut self,
        value: crate::TMPro::HorizontalAlignmentOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalAlignment", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalMapping(
        &mut self,
        value: crate::TMPro::TextureMappingOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalMapping", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ignoreVisibility(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ignoreVisibility", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isOrthographic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isOrthographic", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isOverlay(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isOverlay", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isRightToLeftText(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isRightToLeftText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isTextObjectScaleStatic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isTextObjectScaleStatic", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isUsingLegacyAnimationComponent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isUsingLegacyAnimationComponent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isVolumetricText(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isVolumetricText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_lineSpacing(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lineSpacing", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_lineSpacingAdjustment(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lineSpacingAdjustment", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_linkedTextComponent(
        &mut self,
        value: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_linkedTextComponent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_mappingUvLineOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mappingUvLineOffset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_margin(
        &mut self,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_margin", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_maxVisibleCharacters(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxVisibleCharacters", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_maxVisibleLines(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxVisibleLines", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_maxVisibleWords(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxVisibleWords", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_outlineColor(
        &mut self,
        value: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_outlineColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_outlineWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_outlineWidth", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_overflowMode(
        &mut self,
        value: crate::TMPro::TextOverflowModes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overflowMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_overrideColorTags(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overrideColorTags", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_pageToDisplay(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pageToDisplay", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_paragraphSpacing(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_paragraphSpacing", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_parseCtrlCharacters(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_parseCtrlCharacters", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_renderMode(
        &mut self,
        value: crate::TMPro::TextRenderFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_renderMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_richText(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_richText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_spriteAsset(
        &mut self,
        value: *mut crate::TMPro::TMP_SpriteAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteAsset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_styleSheet(
        &mut self,
        value: *mut crate::TMPro::TMP_StyleSheet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_styleSheet", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_text(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_textPreprocessor(
        &mut self,
        value: *mut crate::TMPro::ITextPreprocessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_textPreprocessor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_textStyle(
        &mut self,
        value: *mut crate::TMPro::TMP_Style,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_textStyle", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_tintAllSprites(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tintAllSprites", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_useMaxVisibleDescender(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useMaxVisibleDescender", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_vertexBufferAutoSizeReduction(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vertexBufferAutoSizeReduction", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_verticalAlignment(
        &mut self,
        value: crate::TMPro::VerticalAlignmentOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalAlignment", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_verticalMapping(
        &mut self,
        value: crate::TMPro::TextureMappingOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalMapping", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_wordSpacing(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_wordSpacing", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_wordWrappingRatios(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_wordWrappingRatios", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_Text")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_Text {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_Text+CharacterSubstitution")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_Text_CharacterSubstitution {
    pub index: i32,
    pub unicode: u32,
}
#[cfg(feature = "TMPro+TMP_Text+CharacterSubstitution")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Text_CharacterSubstitution => "TMPro"
    ."TMP_Text/CharacterSubstitution"
);
#[cfg(feature = "TMPro+TMP_Text+CharacterSubstitution")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::TMP_Text_CharacterSubstitution {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_Text+CharacterSubstitution")]
impl crate::TMPro::TMP_Text_CharacterSubstitution {
    pub fn _ctor(
        &mut self,
        index: i32,
        unicode: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (index, unicode),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_Text+SpecialCharacter")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_Text_SpecialCharacter {
    pub character: *mut crate::TMPro::TMP_Character,
    pub fontAsset: *mut crate::TMPro::TMP_FontAsset,
    pub material: *mut crate::UnityEngine::Material,
    pub materialIndex: i32,
}
#[cfg(feature = "TMPro+TMP_Text+SpecialCharacter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Text_SpecialCharacter => "TMPro"
    ."TMP_Text/SpecialCharacter"
);
#[cfg(feature = "TMPro+TMP_Text+SpecialCharacter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::TMP_Text_SpecialCharacter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_Text+SpecialCharacter")]
impl crate::TMPro::TMP_Text_SpecialCharacter {
    pub fn _ctor(
        &mut self,
        character: *mut crate::TMPro::TMP_Character,
        materialIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (character, materialIndex),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_Text+TextBackingContainer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_Text_TextBackingContainer {
    pub m_Array: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub m_Count: i32,
}
#[cfg(feature = "TMPro+TMP_Text+TextBackingContainer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Text_TextBackingContainer => "TMPro"
    ."TMP_Text/TextBackingContainer"
);
#[cfg(feature = "TMPro+TMP_Text+TextBackingContainer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::TMP_Text_TextBackingContainer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_Text+TextBackingContainer")]
impl crate::TMPro::TMP_Text_TextBackingContainer {
    pub fn Resize(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Resize",
            (_cordl_size),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_size),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Capacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Capacity",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Count(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Count",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (index, value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_Text+TextInputSources")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TMP_Text_TextInputSources {
    SetText = 1i32,
    SetTextArray = 2i32,
    TextInputBox = 0i32,
    TextString = 3i32,
}
#[cfg(feature = "TMPro+TMP_Text+TextInputSources")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Text_TextInputSources => "TMPro"
    ."TMP_Text/TextInputSources"
);
#[cfg(feature = "TMPro+TMP_Text+UnicodeChar")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_Text_UnicodeChar {
    pub unicode: i32,
    pub stringIndex: i32,
    pub length: i32,
}
#[cfg(feature = "TMPro+TMP_Text+UnicodeChar")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Text_UnicodeChar => "TMPro"
    ."TMP_Text/UnicodeChar"
);
#[cfg(feature = "TMPro+TMP_Text+UnicodeChar")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_Text_UnicodeChar {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_Text+UnicodeChar")]
impl crate::TMPro::TMP_Text_UnicodeChar {}
