#[cfg(feature = "UnityEngine+TextCore+Text+FontAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct FontAsset {
    __cordl_parent: crate::UnityEngine::TextCore::Text::TextAsset,
    pub m_SourceFontFileGUID: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_fontAssetCreationEditorSettings: crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings,
    pub m_SourceFontFile: *mut crate::UnityEngine::Font,
    pub m_SourceFontFilePath: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_AtlasPopulationMode: crate::UnityEngine::TextCore::Text::AtlasPopulationMode,
    pub InternalDynamicOS: bool,
    pub m_FaceInfo: crate::UnityEngine::TextCore::FaceInfo,
    pub m_FamilyNameHashCode: i32,
    pub m_StyleNameHashCode: i32,
    pub m_GlyphTable: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Glyph,
    >,
    pub m_GlyphLookupDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        *mut crate::UnityEngine::TextCore::Glyph,
    >,
    pub m_CharacterTable: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Text::Character,
    >,
    pub m_CharacterLookupDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        *mut crate::UnityEngine::TextCore::Text::Character,
    >,
    pub m_AtlasTexture: *mut crate::UnityEngine::Texture2D,
    pub m_AtlasTextures: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Texture2D,
    >,
    pub m_AtlasTextureIndex: i32,
    pub m_IsMultiAtlasTexturesEnabled: bool,
    pub m_ClearDynamicDataOnBuild: bool,
    pub m_AtlasWidth: i32,
    pub m_AtlasHeight: i32,
    pub m_AtlasPadding: i32,
    pub m_AtlasRenderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
    pub m_UsedGlyphRects: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::TextCore::GlyphRect,
    >,
    pub m_FreeGlyphRects: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::TextCore::GlyphRect,
    >,
    pub m_FontFeatureTable: *mut crate::UnityEngine::TextCore::Text::FontFeatureTable,
    pub m_FallbackFontAssetTable: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Text::FontAsset,
    >,
    pub m_FontWeightTable: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::TextCore::Text::FontWeightPair,
    >,
    pub m_RegularStyleWeight: f32,
    pub m_RegularStyleSpacing: f32,
    pub m_BoldStyleWeight: f32,
    pub m_BoldStyleSpacing: f32,
    pub m_ItalicStyleSlant: u8,
    pub m_TabMultiple: u8,
    pub IsFontAssetLookupTablesDirty: bool,
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
        *mut crate::UnityEngine::TextCore::Text::Character,
    >,
    pub m_CharactersToAddLookup: *mut crate::System::Collections::Generic::HashSet_1<
        u32,
    >,
    pub s_MissingCharacterList: *mut crate::System::Collections::Generic::List_1<u32>,
    pub m_MissingUnicodesFromFontFile: *mut crate::System::Collections::Generic::HashSet_1<
        u32,
    >,
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::FontAsset =>
    "UnityEngine.TextCore.Text"."FontAsset"
);
#[cfg(feature = "UnityEngine+TextCore+Text+FontAsset")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::FontAsset {
    type Target = crate::UnityEngine::TextCore::Text::TextAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAsset")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::FontAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAsset")]
impl crate::UnityEngine::TextCore::Text::FontAsset {
    #[cfg(feature = "UnityEngine+TextCore+Text+FontAsset+__c")]
    pub type __c = crate::UnityEngine::TextCore::Text::FontAsset___c;
    pub fn AddCharacterToLookupCache(
        &mut self,
        unicode: u32,
        character: *mut crate::UnityEngine::TextCore::Text::Character,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCharacterToLookupCache", (unicode, character))?;
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
    pub fn ClearFontAssetDataInternal(
        &mut self,
        clearFontFeatures: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearFontAssetDataInternal", (clearFontFeatures))?;
        Ok(__cordl_ret)
    }
    pub fn ClearFontAssetTables(
        &mut self,
        clearFontFeatures: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearFontAssetTables", (clearFontFeatures))?;
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
    pub fn DestroyAtlasTextures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyAtlasTextures", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGlyphIndex(&mut self, unicode: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetGlyphIndex", (unicode))?;
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
    pub fn HasCharacter_u32__cordl_bool__cordl_bool2(
        &mut self,
        character: u32,
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
    pub fn HasCharacters_ByRefMut0(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
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
        text: *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn HasCharacters_Il2CppString2(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasCharacters", (text))?;
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
    pub fn InitializeLigatureSubstitutionLookupDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeLigatureSubstitutionLookupDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeMarkToBaseAdjustmentRecordsLookupDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeMarkToBaseAdjustmentRecordsLookupDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeMarkToMarkAdjustmentRecordsLookupDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeMarkToMarkAdjustmentRecordsLookupDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadFontFace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = __cordl_object
            .invoke("LoadFontFace", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
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
    pub fn TryAddCharacterInternal(
        &mut self,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::TextCore::Text::Character,
        >,
        shouldGetFontFeatures: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryAddCharacterInternal",
                (unicode, character, shouldGetFontFeatures),
            )?;
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
    pub fn TryAddCharacters_Il2CppString_ByRefMut__cordl_bool3(
        &mut self,
        characters: *mut quest_hook::libil2cpp::Il2CppString,
        missingCharacters: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
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
    pub fn TryAddCharacters_Il2CppString__cordl_bool2(
        &mut self,
        characters: *mut quest_hook::libil2cpp::Il2CppString,
        includeFontFeatures: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryAddCharacters", (characters, includeFontFeatures))?;
        Ok(__cordl_ret)
    }
    pub fn TryAddGlyphInternal(
        &mut self,
        glyphIndex: u32,
        glyph: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::TextCore::Glyph>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryAddGlyphInternal", (glyphIndex, glyph))?;
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
    pub fn TryGetCharacter_and_QueueRenderToTexture(
        &mut self,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::TextCore::Text::Character,
        >,
        shouldGetFontFeatures: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryGetCharacter_and_QueueRenderToTexture",
                (unicode, character, shouldGetFontFeatures),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAllFontFeatures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAllFontFeatures", ())?;
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
    pub fn get_atlasHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_atlasHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_atlasPadding(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_atlasPadding", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_atlasPopulationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::Text::AtlasPopulationMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextCore::Text::AtlasPopulationMode = __cordl_object
            .invoke("get_atlasPopulationMode", ())?;
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
    pub fn get_atlasTextureCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_atlasTextureCount", ())?;
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
    pub fn get_atlasWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_atlasWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_boldStyleSpacing(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_boldStyleSpacing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_boldStyleWeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_boldStyleWeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characterLookupTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::UnityEngine::TextCore::Text::Character,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::UnityEngine::TextCore::Text::Character,
        > = __cordl_object.invoke("get_characterLookupTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characterTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::Character,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::Character,
        > = __cordl_object.invoke("get_characterTable", ())?;
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
    pub fn get_fallbackFontAssetTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::FontAsset,
        > = __cordl_object.invoke("get_fallbackFontAssetTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_familyNameHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_familyNameHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontAssetCreationEditorSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings = __cordl_object
            .invoke("get_fontAssetCreationEditorSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontFeatureTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::TextCore::Text::FontFeatureTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextCore::Text::FontFeatureTable = __cordl_object
            .invoke("get_fontFeatureTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontWeightTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::TextCore::Text::FontWeightPair,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::TextCore::Text::FontWeightPair,
        > = __cordl_object.invoke("get_fontWeightTable", ())?;
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
    pub fn get_italicStyleSlant(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_italicStyleSlant", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_regularStyleSpacing(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_regularStyleSpacing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_regularStyleWeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_regularStyleWeight", ())?;
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
    pub fn get_styleNameHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_styleNameHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_tabMultiple(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_tabMultiple", ())?;
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
    pub fn set_atlasPopulationMode(
        &mut self,
        value: crate::UnityEngine::TextCore::Text::AtlasPopulationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlasPopulationMode", (value))?;
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
    pub fn set_boldStyleSpacing(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_boldStyleSpacing", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_boldStyleWeight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_boldStyleWeight", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_characterTable(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::Character,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_characterTable", (value))?;
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
    pub fn set_fallbackFontAssetTable(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fallbackFontAssetTable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_familyNameHashCode(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_familyNameHashCode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontAssetCreationEditorSettings(
        &mut self,
        value: crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontAssetCreationEditorSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontFeatureTable(
        &mut self,
        value: *mut crate::UnityEngine::TextCore::Text::FontFeatureTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontFeatureTable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fontWeightTable(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::TextCore::Text::FontWeightPair,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontWeightTable", (value))?;
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
    pub fn set_italicStyleSlant(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_italicStyleSlant", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_regularStyleSpacing(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_regularStyleSpacing", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_regularStyleWeight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_regularStyleWeight", (value))?;
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
    pub fn set_styleNameHashCode(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_styleNameHashCode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_tabMultiple(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tabMultiple", (value))?;
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
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::FontAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
