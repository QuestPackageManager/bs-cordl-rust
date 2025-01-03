#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct FontEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::LowLevel::FontEngine =>
    "UnityEngine.TextCore.LowLevel"."FontEngine"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl std::ops::Deref for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl crate::UnityEngine::TextCore::LowLevel::FontEngine {
    pub fn GenericListToMarshallingArray<T>(
        srcList: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::List_1<T>,
        >,
        dstArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenericListToMarshallingArray", (srcList, dstArray))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceInfo() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::FaceInfo,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::FaceInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFaceInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceInfo_Internal(
        faceInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::TextCore::FaceInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFaceInfo_Internal", (faceInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlyphIndex(unicode: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlyphIndex", (unicode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlyphPairAdjustmentRecords(
        glyphIndexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u32>,
        >,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlyphPairAdjustmentRecords", (glyphIndexes, recordCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlyphPairAdjustmentTable(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlyphPairAdjustmentTable", (glyphIndexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPairAdjustmentRecordsFromMarshallingArray(
        glyphPairAdjustmentRecords: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetPairAdjustmentRecordsFromMarshallingArray",
                (glyphPairAdjustmentRecords),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeFontEngine() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeFontEngine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeFontEngine_Internal() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeFontEngine_Internal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Font_i32_1(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        pointSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFontFace", (font, pointSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Font_i32_i32_2(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFontFace", (font, pointSize, faceIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Il2CppString_Il2CppString_i32_3(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFontFace", (familyName, styleName, pointSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Il2CppString_i32_i32_0(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFontFace", (filePath, pointSize, faceIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_And_FaceIndex_Internal(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadFontFace_With_Size_And_FaceIndex_Internal",
                (filePath, pointSize, faceIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_FromFont_Internal(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        pointSize: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFontFace_With_Size_FromFont_Internal", (font, pointSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal",
                (font, pointSize, faceIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal",
                (familyName, styleName, pointSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulatePairAdjustmentRecordMarshallingArray_from_KernTable(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PopulatePairAdjustmentRecordMarshallingArray_from_KernTable",
                (glyphIndexes, recordCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetAtlasTexture(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetAtlasTexture", (texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMarshallingArraySize<T>(
        marshallingArray: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<T>,
        >,
        recordCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMarshallingArraySize", (marshallingArray, recordCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTextureUploadMode(
        shouldUploadImmediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTextureUploadMode", (shouldUploadImmediately))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryAddGlyphToTexture(
        glyphIndex: u32,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        usedGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        glyph: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::TextCore::Glyph>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryAddGlyphToTexture",
                (
                    glyphIndex,
                    padding,
                    packingMode,
                    freeGlyphRects,
                    usedGlyphRects,
                    renderMode,
                    texture,
                    glyph,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryAddGlyphToTexture_Internal(
        glyphIndex: u32,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        freeGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        usedGlyphRects: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        usedGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        glyph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryAddGlyphToTexture_Internal",
                (
                    glyphIndex,
                    padding,
                    packingMode,
                    freeGlyphRects,
                    freeGlyphRectCount,
                    usedGlyphRects,
                    usedGlyphRectCount,
                    renderMode,
                    texture,
                    glyph,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryAddGlyphsToTexture(
        glyphIndexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u32>,
        >,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        usedGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        glyphs: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::TextCore::Glyph,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryAddGlyphsToTexture",
                (
                    glyphIndexes,
                    padding,
                    packingMode,
                    freeGlyphRects,
                    usedGlyphRects,
                    renderMode,
                    texture,
                    glyphs,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryAddGlyphsToTexture_Internal(
        glyphIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        freeGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        usedGlyphRects: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        usedGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        glyphs: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
            >,
        >,
        glyphCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryAddGlyphsToTexture_Internal",
                (
                    glyphIndex,
                    padding,
                    packingMode,
                    freeGlyphRects,
                    freeGlyphRectCount,
                    usedGlyphRects,
                    usedGlyphRectCount,
                    renderMode,
                    texture,
                    glyphs,
                    glyphCount,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetGlyphWithIndexValue(
        glyphIndex: u32,
        flags: crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
        glyph: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::TextCore::Glyph>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetGlyphWithIndexValue", (glyphIndex, flags, glyph))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetGlyphWithIndexValue_Internal(
        glyphIndex: u32,
        loadFlags: crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
        glyphStruct: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryGetGlyphWithIndexValue_Internal",
                (glyphIndex, loadFlags, glyphStruct),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetGlyphWithUnicodeValue(
        unicode: u32,
        flags: crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
        glyph: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::TextCore::Glyph>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetGlyphWithUnicodeValue", (unicode, flags, glyph))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetGlyphWithUnicodeValue_Internal(
        unicode: u32,
        loadFlags: crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
        glyphStruct: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryGetGlyphWithUnicodeValue_Internal",
                (unicode, loadFlags, glyphStruct),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSystemFontReference(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fontRef: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::FontReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetSystemFontReference", (familyName, styleName, fontRef))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSystemFontReference_Internal(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fontRef: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::FontReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryGetSystemFontReference_Internal",
                (familyName, styleName, fontRef),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
