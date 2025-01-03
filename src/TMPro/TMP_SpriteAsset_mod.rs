#[cfg(feature = "TMPro+TMP_SpriteAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_SpriteAsset {
    __cordl_parent: crate::TMPro::TMP_Asset,
    pub m_NameLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<i32, i32>,
    >,
    pub m_GlyphIndexLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<u32, i32>,
    >,
    pub m_Version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_FaceInfo: crate::UnityEngine::TextCore::FaceInfo,
    pub spriteSheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub m_SpriteCharacterTable: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::TMPro::TMP_SpriteCharacter,
        >,
    >,
    pub m_SpriteCharacterLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::TMPro::TMP_SpriteCharacter,
        >,
    >,
    pub m_SpriteGlyphTable: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::TMPro::TMP_SpriteGlyph>,
    >,
    pub m_SpriteGlyphLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::TMPro::TMP_SpriteGlyph,
        >,
    >,
    pub spriteInfoList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::TMPro::TMP_Sprite>,
    >,
    pub fallbackSpriteAssets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::TMPro::TMP_SpriteAsset>,
    >,
    pub m_IsSpriteAssetLookupTablesDirty: bool,
}
#[cfg(feature = "TMPro+TMP_SpriteAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_SpriteAsset => "TMPro"
    ."TMP_SpriteAsset"
);
#[cfg(feature = "TMPro+TMP_SpriteAsset")]
impl std::ops::Deref for crate::TMPro::TMP_SpriteAsset {
    type Target = crate::TMPro::TMP_Asset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_SpriteAsset")]
impl std::ops::DerefMut for crate::TMPro::TMP_SpriteAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_SpriteAsset")]
impl crate::TMPro::TMP_SpriteAsset {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultSpriteMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("GetDefaultSpriteMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteIndexFromHashcode(
        &mut self,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetSpriteIndexFromHashcode", (hashCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteIndexFromName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSpriteIndexFromName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteIndexFromUnicode(
        &mut self,
        unicode: u32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetSpriteIndexFromUnicode", (unicode))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SearchForSpriteByHashCode(
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        hashCode: i32,
        includeFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByHashCode",
                (spriteAsset, hashCode, includeFallbacks, spriteIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForSpriteByHashCodeInternal_List_1_0(
        spriteAssets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::TMPro::TMP_SpriteAsset,
            >,
        >,
        hashCode: i32,
        searchFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByHashCodeInternal",
                (spriteAssets, hashCode, searchFallbacks, spriteIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForSpriteByHashCodeInternal_TMP_SpriteAsset1(
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        hashCode: i32,
        searchFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByHashCodeInternal",
                (spriteAsset, hashCode, searchFallbacks, spriteIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForSpriteByUnicode(
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        unicode: u32,
        includeFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByUnicode",
                (spriteAsset, unicode, includeFallbacks, spriteIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForSpriteByUnicodeInternal_List_1_0(
        spriteAssets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::TMPro::TMP_SpriteAsset,
            >,
        >,
        unicode: u32,
        includeFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByUnicodeInternal",
                (spriteAssets, unicode, includeFallbacks, spriteIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForSpriteByUnicodeInternal_TMP_SpriteAsset1(
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        unicode: u32,
        includeFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByUnicodeInternal",
                (spriteAsset, unicode, includeFallbacks, spriteIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SortCharacterTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortCharacterTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SortGlyphAndCharacterTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortGlyphAndCharacterTables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SortGlyphTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortGlyphTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLookupTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLookupTables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpgradeSpriteAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpgradeSpriteAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_faceInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::FaceInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextCore::FaceInfo = __cordl_object
            .invoke("get_faceInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spriteCharacterLookupTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                *mut crate::TMPro::TMP_SpriteCharacter,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                *mut crate::TMPro::TMP_SpriteCharacter,
            >,
        > = __cordl_object.invoke("get_spriteCharacterLookupTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spriteCharacterTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::TMPro::TMP_SpriteCharacter,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::TMPro::TMP_SpriteCharacter,
            >,
        > = __cordl_object.invoke("get_spriteCharacterTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spriteGlyphTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::TMPro::TMP_SpriteGlyph,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::TMPro::TMP_SpriteGlyph,
            >,
        > = __cordl_object.invoke("get_spriteGlyphTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_version", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_spriteCharacterLookupTable(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                *mut crate::TMPro::TMP_SpriteCharacter,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteCharacterLookupTable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_spriteCharacterTable(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::TMPro::TMP_SpriteCharacter,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteCharacterTable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_spriteGlyphTable(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::TMPro::TMP_SpriteGlyph,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteGlyphTable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_version(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_version", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_SpriteAsset")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_SpriteAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
