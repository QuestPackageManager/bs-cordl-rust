#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TextGeneratorUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::TextGeneratorUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextGeneratorUtilities";
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
#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextGeneratorUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextGeneratorUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
impl crate::UnityEngine::TextCore::Text::TextGeneratorUtilities {
    pub fn AdjustLineOffset(
        startIndex: i32,
        endIndex: i32,
        offset: f32,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustLineOffset", (startIndex, endIndex, offset, textInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approximately(a: f32, b: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approximately", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToFloat_ByRefMut1(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
        lastIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToFloat", (chars, startIndex, length, lastIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToFloat_Il2CppArray_i32_i32_0(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToFloat", (chars, startIndex, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToUTF32(
        highSurrogate: u32,
        lowSurrogate: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToUTF32", (highSurrogate, lowSurrogate))?;
        Ok(__cordl_ret.into())
    }
    pub fn FillCharacterVertexBuffers(
        i: i32,
        convertToLinearSpace: bool,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FillCharacterVertexBuffers",
                (i, convertToLinearSpace, generationSettings, textInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FillSpriteVertexBuffers(
        i: i32,
        convertToLinearSpace: bool,
        generationSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FillSpriteVertexBuffers",
                (i, convertToLinearSpace, generationSettings, textInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GammaToLinear_Color32_0(
        c: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GammaToLinear", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn GammaToLinear_u8_1(value: u8) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GammaToLinear", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeParameters(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
        parameters: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttributeParameters", (chars, startIndex, length, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkupTagHashCode_Il2CppArray1(
        styleDefinition: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        >,
        readIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMarkupTagHashCode", (styleDefinition, readIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkupTagHashCode_TextBackingContainer0(
        styleDefinition: crate::UnityEngine::TextCore::Text::TextBackingContainer,
        readIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMarkupTagHashCode", (styleDefinition, readIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStyle(
        generationSetting: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextStyle>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextStyle,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStyle", (generationSetting, hashCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStyleHashCode_ByRefMut_i32_ByRefMut0(
        text: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        >,
        index: i32,
        closeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStyleHashCode", (text, index, closeIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStyleHashCode_ByRefMut_i32_ByRefMut1(
        text: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::Text::TextBackingContainer,
        >,
        index: i32,
        closeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStyleHashCode", (text, index, closeIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUTF16_Il2CppArray0(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUTF16", (text, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUTF16_TextBackingContainer1(
        text: crate::UnityEngine::TextCore::Text::TextBackingContainer,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUTF16", (text, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUTF32_Il2CppArray0(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUTF32", (text, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUTF32_TextBackingContainer1(
        text: crate::UnityEngine::TextCore::Text::TextBackingContainer,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUTF32", (text, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn HexCharsToColor_Il2CppArray_i32_0(
        hexChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        tagCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HexCharsToColor", (hexChars, tagCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn HexCharsToColor_i32_1(
        hexChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HexCharsToColor", (hexChars, startIndex, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn HexToInt(hex: char) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HexToInt", (hex))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertClosingStyleTag(
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InsertClosingStyleTag",
                (
                    charBuffer,
                    writeIndex,
                    textStyleStackDepth,
                    textStyleStacks,
                    generationSettings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertClosingTextStyle(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextStyle>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InsertClosingTextStyle",
                (
                    style,
                    charBuffer,
                    writeIndex,
                    textStyleStackDepth,
                    textStyleStacks,
                    generationSettings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertOpeningStyleTag(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextStyle>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InsertOpeningStyleTag",
                (
                    style,
                    charBuffer,
                    writeIndex,
                    textStyleStackDepth,
                    textStyleStacks,
                    generationSettings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertOpeningTextStyle(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextStyle>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InsertOpeningTextStyle",
                (
                    style,
                    charBuffer,
                    writeIndex,
                    textStyleStackDepth,
                    textStyleStacks,
                    generationSettings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertTextStyleInTextProcessingArray(
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        styleDefinition: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        >,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InsertTextStyleInTextProcessingArray",
                (
                    charBuffer,
                    writeIndex,
                    styleDefinition,
                    textStyleStackDepth,
                    textStyleStacks,
                    generationSettings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBaseGlyph(c: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBaseGlyph", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBitmapRendering(
        glyphRenderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBitmapRendering", (glyphRenderMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCJK(c: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCJK", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHangul(c: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHangul", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUTF16(
        text: crate::UnityEngine::TextCore::Text::TextBackingContainer,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUTF16", (text, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUTF32(
        text: crate::UnityEngine::TextCore::Text::TextBackingContainer,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUTF32", (text, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn LegacyAlignmentToNewAlignment(
        anchor: crate::UnityEngine::TextAnchor,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::Text::TextAlignment,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::Text::TextAlignment = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LegacyAlignmentToNewAlignment", (anchor))?;
        Ok(__cordl_ret.into())
    }
    pub fn LegacyStyleToNewStyle(
        fontStyle: crate::UnityEngine::FontStyle,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::Text::FontStyles> {
        let __cordl_ret: crate::UnityEngine::TextCore::Text::FontStyles = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LegacyStyleToNewStyle", (fontStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn MinAlpha(
        c1: crate::UnityEngine::Color,
        c2: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MinAlpha", (c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn PackUV(
        x: f32,
        y: f32,
        scale: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PackUV", (x, y, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceClosingStyleTag(
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReplaceClosingStyleTag",
                (
                    charBuffer,
                    writeIndex,
                    textStyleStackDepth,
                    textStyleStacks,
                    generationSettings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceOpeningStyleTag_ByRefMut_i32_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut0(
        sourceText: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::Text::TextBackingContainer,
        >,
        srcIndex: i32,
        srcOffset: quest_hook::libil2cpp::ByRefMut<i32>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReplaceOpeningStyleTag",
                (
                    sourceText,
                    srcIndex,
                    srcOffset,
                    charBuffer,
                    writeIndex,
                    textStyleStackDepth,
                    textStyleStacks,
                    generationSettings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceOpeningStyleTag_ByRefMut_i32_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut1(
        sourceText: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        >,
        srcIndex: i32,
        srcOffset: quest_hook::libil2cpp::ByRefMut<i32>,
        charBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingElement,
                >,
            >,
        >,
        writeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStackDepth: quest_hook::libil2cpp::ByRefMut<i32>,
        textStyleStacks: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::Text::TextProcessingStack_1<i32>,
                >,
            >,
        >,
        generationSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReplaceOpeningStyleTag",
                (
                    sourceText,
                    srcIndex,
                    srcOffset,
                    charBuffer,
                    writeIndex,
                    textStyleStackDepth,
                    textStyleStacks,
                    generationSettings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ResizeInternalArray_ByRefMut0<T>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResizeInternalArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResizeInternalArray_i32_1<T>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResizeInternalArray", (array, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResizeLineExtents(
        _cordl_size: i32,
        textInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResizeLineExtents", (_cordl_size, textInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperASCIIFast__cordl_char0(
        c: char,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUpperASCIIFast", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperASCIIFast_u32_1(c: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUpperASCIIFast", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperFast(c: char) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUpperFast", (c))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGeneratorUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextGeneratorUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
