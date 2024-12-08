#[cfg(feature = "TMPro+TMP_FontAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_FontAsset {
    __cordl_parent: crate::TMPro::TMP_Asset,
    pub m_Version: *mut crate::System::String,
    pub m_SourceFontFileGUID: *mut crate::System::String,
    pub m_SourceFontFile: *mut crate::UnityEngine::Font,
    pub m_AtlasPopulationMode: crate::TMPro::AtlasPopulationMode,
    pub m_FaceInfo: crate::UnityEngine::TextCore::FaceInfo,
    pub m_GlyphTable: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Glyph,
    >,
    pub m_GlyphLookupDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        *mut crate::UnityEngine::TextCore::Glyph,
    >,
    pub m_CharacterTable: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_Character,
    >,
    pub m_CharacterLookupDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        *mut crate::TMPro::TMP_Character,
    >,
    pub m_AtlasTexture: *mut crate::UnityEngine::Texture2D,
    pub m_AtlasTextures: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Texture2D,
    >,
    pub m_AtlasTextureIndex: i32,
    pub m_IsMultiAtlasTexturesEnabled: bool,
    pub m_ClearDynamicDataOnBuild: bool,
    pub m_UsedGlyphRects: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::TextCore::GlyphRect,
    >,
    pub m_FreeGlyphRects: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::TextCore::GlyphRect,
    >,
    pub m_fontInfo: *mut crate::TMPro::FaceInfo_Legacy,
    pub atlas: *mut crate::UnityEngine::Texture2D,
    pub m_AtlasWidth: i32,
    pub m_AtlasHeight: i32,
    pub m_AtlasPadding: i32,
    pub m_AtlasRenderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
    pub m_glyphInfoList: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_Glyph,
    >,
    pub m_KerningTable: *mut crate::TMPro::KerningTable,
    pub m_FontFeatureTable: *mut crate::TMPro::TMP_FontFeatureTable,
    pub fallbackFontAssets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_FontAsset,
    >,
    pub m_FallbackFontAssetTable: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_FontAsset,
    >,
    pub m_CreationSettings: crate::TMPro::FontAssetCreationSettings,
    pub m_FontWeightTable: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::TMPro::TMP_FontWeightPair,
    >,
    pub fontWeights: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::TMPro::TMP_FontWeightPair,
    >,
    pub normalStyle: f32,
    pub normalSpacingOffset: f32,
    pub boldStyle: f32,
    pub boldSpacing: f32,
    pub italicStyle: u8,
    pub tabSize: u8,
    pub IsFontAssetLookupTablesDirty: bool,
    pub FallbackSearchQueryLookup: *mut crate::System::Collections::Generic::HashSet_1<
        i32,
    >,
    pub m_GlyphsToRender: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Glyph,
    >,
    pub m_GlyphsRendered: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Glyph,
    >,
    pub m_GlyphIndexList: *mut crate::System::Collections::Generic::List_1<u32>,
    pub m_GlyphIndexListNewlyAdded: *mut crate::System::Collections::Generic::List_1<
        u32,
    >,
    pub m_GlyphsToAdd: *mut crate::System::Collections::Generic::List_1<u32>,
    pub m_GlyphsToAddLookup: *mut crate::System::Collections::Generic::HashSet_1<u32>,
    pub m_CharactersToAdd: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_Character,
    >,
    pub m_CharactersToAddLookup: *mut crate::System::Collections::Generic::HashSet_1<
        u32,
    >,
    pub s_MissingCharacterList: *mut crate::System::Collections::Generic::List_1<u32>,
    pub m_MissingUnicodesFromFontFile: *mut crate::System::Collections::Generic::HashSet_1<
        u32,
    >,
}
#[cfg(feature = "TMPro+TMP_FontAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_FontAsset => "TMPro"."TMP_FontAsset"
);
#[cfg(feature = "TMPro+TMP_FontAsset")]
impl std::ops::Deref for crate::TMPro::TMP_FontAsset {
    type Target = crate::TMPro::TMP_Asset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontAsset")]
impl std::ops::DerefMut for crate::TMPro::TMP_FontAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontAsset")]
impl crate::TMPro::TMP_FontAsset {
    #[cfg(feature = "TMPro+TMP_FontAsset+__c")]
    pub type __c = crate::TMPro::TMP_FontAsset___c;
    pub fn set_isMultiAtlasTexturesEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isMultiAtlasTexturesEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpgradeFontAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpgradeFontAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_clearDynamicDataOnBuild(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clearDynamicDataOnBuild", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_usedGlyphRects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::GlyphRect,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::GlyphRect,
        > = __cordl_object.invoke("get_usedGlyphRects", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_freeGlyphRects(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::GlyphRect,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_freeGlyphRects", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SortGlyphTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortGlyphTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupNewAtlasTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupNewAtlasTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearFontAssetData(
        &mut self,
        setAtlasSizeToZero: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearFontAssetData", (setAtlasSizeToZero))?;
        Ok(__cordl_ret)
    }
    pub fn ClearFontAssetTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearFontAssetTables", ())?;
        Ok(__cordl_ret)
    }
    pub fn SortFontFeatureTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortFontFeatureTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_atlasPopulationMode(
        &mut self,
        value: crate::TMPro::AtlasPopulationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlasPopulationMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_atlasTextures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Texture2D>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Texture2D,
        > = __cordl_object.invoke("get_atlasTextures", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fallbackFontAssetTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::TMPro::TMP_FontAsset,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::TMPro::TMP_FontAsset,
        > = __cordl_object.invoke("get_fallbackFontAssetTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_creationSettings(
        &mut self,
        value: crate::TMPro::FontAssetCreationSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_creationSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HasCharacter_i32_0(
        &mut self,
        character: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasCharacter", (character))?;
        Ok(__cordl_ret)
    }
    pub fn HasCharacter__cordl_char__cordl_bool__cordl_bool1(
        &mut self,
        character: char,
        searchFallbacks: bool,
        tryAddCharacter: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasCharacter", (character, searchFallbacks, tryAddCharacter))?;
        Ok(__cordl_ret)
    }
    pub fn ReadFontAssetDefinition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadFontAssetDefinition", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_characterTable(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::TMPro::TMP_Character,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_characterTable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontFeatureTable(
        &mut self,
        value: *mut crate::TMPro::TMP_FontFeatureTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontFeatureTable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeGlyphPaidAdjustmentRecordsLookupDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeGlyphPaidAdjustmentRecordsLookupDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_atlasWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_atlasWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateFontAssetData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFontAssetData", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasCharacter_Internal(
        &mut self,
        character: u32,
        searchFallbacks: bool,
        tryAddCharacter: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "HasCharacter_Internal",
                (character, searchFallbacks, tryAddCharacter),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_atlasTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("get_atlasTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_atlasWidth(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlasWidth", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AddSynthesizedCharacter(
        &mut self,
        unicode: u32,
        isFontFaceLoaded: bool,
        addImmediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSynthesizedCharacter",
                (unicode, isFontFaceLoaded, addImmediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TryAddGlyphsToAtlasTextures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryAddGlyphsToAtlasTextures", ())?;
        Ok(__cordl_ret)
    }
    pub fn CopyListDataToArray<T>(
        &mut self,
        srcList: *mut crate::System::Collections::Generic::List_1<T>,
        dstArray: quest_hook::libil2cpp::ByRefMut<
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
            .invoke("CopyListDataToArray", (srcList, dstArray))?;
        Ok(__cordl_ret)
    }
    pub fn AddSynthesizedCharactersAndFaceMetrics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSynthesizedCharactersAndFaceMetrics", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryAddCharacterInternal(
        &mut self,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<*mut crate::TMPro::TMP_Character>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryAddCharacterInternal", (unicode, character))?;
        Ok(__cordl_ret)
    }
    pub fn set_faceInfo(
        &mut self,
        value: crate::UnityEngine::TextCore::FaceInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_faceInfo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ClearAtlasTextures(
        &mut self,
        setAtlasSizeToZero: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearAtlasTextures", (setAtlasSizeToZero))?;
        Ok(__cordl_ret)
    }
    pub fn get_atlasRenderMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode = __cordl_object
            .invoke("get_atlasRenderMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_atlasHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_atlasHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_glyphLookupTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::UnityEngine::TextCore::Glyph,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::UnityEngine::TextCore::Glyph,
        > = __cordl_object.invoke("get_glyphLookupTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_sourceFontFile(
        &mut self,
        value: *mut crate::UnityEngine::Font,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sourceFontFile", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SortCharacterTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortCharacterTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characterTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::TMPro::TMP_Character,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::TMPro::TMP_Character,
        > = __cordl_object.invoke("get_characterTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn SortAllTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortAllTables", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_atlasTextureCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_atlasTextureCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontFeatureTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_FontFeatureTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_FontFeatureTable = __cordl_object
            .invoke("get_fontFeatureTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_freeGlyphRects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::GlyphRect,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::GlyphRect,
        > = __cordl_object.invoke("get_freeGlyphRects", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_version", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryAddCharacters_Il2CppArray__cordl_bool0(
        &mut self,
        unicodes: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        includeFontFeatures: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryAddCharacters", (unicodes, includeFontFeatures))?;
        Ok(__cordl_ret)
    }
    pub fn TryAddCharacters_Il2CppArray_ByRefMut__cordl_bool1(
        &mut self,
        unicodes: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        missingUnicodes: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        >,
        includeFontFeatures: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryAddCharacters",
                (unicodes, missingUnicodes, includeFontFeatures),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TryAddCharacters_String__cordl_bool2(
        &mut self,
        characters: *mut crate::System::String,
        includeFontFeatures: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryAddCharacters", (characters, includeFontFeatures))?;
        Ok(__cordl_ret)
    }
    pub fn TryAddCharacters_String_ByRefMut__cordl_bool3(
        &mut self,
        characters: *mut crate::System::String,
        missingCharacters: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        includeFontFeatures: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryAddCharacters",
                (characters, missingCharacters, includeFontFeatures),
            )?;
        Ok(__cordl_ret)
    }
    pub fn set_usedGlyphRects(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::GlyphRect,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_usedGlyphRects", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_sourceFontFile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Font> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Font = __cordl_object
            .invoke("get_sourceFontFile", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontWeightTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_FontWeightPair>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::TMPro::TMP_FontWeightPair,
        > = __cordl_object.invoke("get_fontWeightTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryAddGlyphsToNewAtlasTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryAddGlyphsToNewAtlasTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddCharacterToLookupCache(
        &mut self,
        unicode: u32,
        character: *mut crate::TMPro::TMP_Character,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCharacterToLookupCache", (unicode, character))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAtlasTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAtlasTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateGlyphAdjustmentRecords_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGlyphAdjustmentRecords", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateGlyphAdjustmentRecords_Il2CppArray1(
        &mut self,
        glyphIndexes: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGlyphAdjustmentRecords", (glyphIndexes))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateGlyphAdjustmentRecords_List_1_2(
        &mut self,
        glyphIndexes: *mut crate::System::Collections::Generic::List_1<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGlyphAdjustmentRecords", (glyphIndexes))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateGlyphAdjustmentRecords_List_1_List_1_3(
        &mut self,
        newGlyphIndexes: *mut crate::System::Collections::Generic::List_1<u32>,
        allGlyphIndexes: *mut crate::System::Collections::Generic::List_1<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGlyphAdjustmentRecords", (newGlyphIndexes, allGlyphIndexes))?;
        Ok(__cordl_ret)
    }
    pub fn ClearFontAssetDataInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearFontAssetDataInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_glyphTable(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Glyph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_glyphTable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isMultiAtlasTexturesEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isMultiAtlasTexturesEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_atlasPadding(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlasPadding", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpgradeGlyphAdjustmentTableToFontFeatureTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpgradeGlyphAdjustmentTableToFontFeatureTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_atlasPadding(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_atlasPadding", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_atlasRenderMode(
        &mut self,
        value: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlasRenderMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_glyphTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Glyph,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Glyph,
        > = __cordl_object.invoke("get_glyphTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasCharacters_ByRefMut0(
        &mut self,
        text: *mut crate::System::String,
        missingCharacters: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::List_1<char>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasCharacters", (text, missingCharacters))?;
        Ok(__cordl_ret)
    }
    pub fn HasCharacters_ByRefMut__cordl_bool__cordl_bool1(
        &mut self,
        text: *mut crate::System::String,
        missingCharacters: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        >,
        searchFallbacks: bool,
        tryAddCharacter: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "HasCharacters",
                (text, missingCharacters, searchFallbacks, tryAddCharacter),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HasCharacters_String2(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasCharacters", (text))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeGlyphLookupDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeGlyphLookupDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_atlasTextures(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Texture2D,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlasTextures", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fallbackFontAssetTable(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::TMPro::TMP_FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fallbackFontAssetTable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_creationSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::FontAssetCreationSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::FontAssetCreationSettings = __cordl_object
            .invoke("get_creationSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_atlasPopulationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::AtlasPopulationMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::AtlasPopulationMode = __cordl_object
            .invoke("get_atlasPopulationMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGlyphIndex(&mut self, unicode: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetGlyphIndex", (unicode))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeDictionaryLookupTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeDictionaryLookupTables", ())?;
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
    pub fn get_characterLookupTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::TMPro::TMP_Character,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::TMPro::TMP_Character,
        > = __cordl_object.invoke("get_characterLookupTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryGetCharacter_and_QueueRenderToTexture(
        &mut self,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<*mut crate::TMPro::TMP_Character>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetCharacter_and_QueueRenderToTexture", (unicode, character))?;
        Ok(__cordl_ret)
    }
    pub fn set_atlasHeight(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlasHeight", (value))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeCharacterLookupDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeCharacterLookupDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_clearDynamicDataOnBuild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_clearDynamicDataOnBuild", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_version(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_version", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_fontInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::FaceInfo_Legacy> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::FaceInfo_Legacy = __cordl_object
            .invoke("get_fontInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_faceInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::FaceInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextCore::FaceInfo = __cordl_object
            .invoke("get_faceInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_fontWeightTable(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_FontWeightPair>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontWeightTable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TMPro+TMP_FontAsset")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_FontAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
