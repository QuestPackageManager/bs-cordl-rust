#[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteAsset {
    __cordl_parent: crate::UnityEngine::TextCore::Text::TextAsset,
    pub m_NameLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<i32, i32>,
    >,
    pub m_GlyphIndexLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<u32, i32>,
    >,
    pub m_FaceInfo: crate::UnityEngine::TextCore::FaceInfo,
    pub m_SpriteAtlasTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub m_SpriteCharacterTable: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::SpriteCharacter,
            >,
        >,
    >,
    pub m_SpriteCharacterLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u32,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::SpriteCharacter,
            >,
        >,
    >,
    pub m_SpriteGlyphTable: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteGlyph>,
        >,
    >,
    pub m_SpriteGlyphLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteGlyph>,
        >,
    >,
    pub fallbackSpriteAssets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteAsset>,
        >,
    >,
    pub m_IsSpriteAssetLookupTablesDirty: bool,
}
#[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::SpriteAsset {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "SpriteAsset";
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
#[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::SpriteAsset {
    type Target = crate::UnityEngine::TextCore::Text::TextAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::SpriteAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset")]
impl crate::UnityEngine::TextCore::Text::SpriteAsset {
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
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
        hashCode: i32,
        includeFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        textSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByHashCode",
                (spriteAsset, hashCode, includeFallbacks, spriteIndex, textSettings),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForSpriteByHashCodeInternal_List_1_0(
        spriteAssets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteAsset,
                >,
            >,
        >,
        hashCode: i32,
        searchFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByHashCodeInternal",
                (spriteAssets, hashCode, searchFallbacks, spriteIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForSpriteByHashCodeInternal_SpriteAsset1(
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
        hashCode: i32,
        searchFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByHashCodeInternal",
                (spriteAsset, hashCode, searchFallbacks, spriteIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForSpriteByUnicode(
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
        unicode: u32,
        includeFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByUnicode",
                (spriteAsset, unicode, includeFallbacks, spriteIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForSpriteByUnicodeInternal_List_1_0(
        spriteAssets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteAsset,
                >,
            >,
        >,
        unicode: u32,
        includeFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SearchForSpriteByUnicodeInternal",
                (spriteAssets, unicode, includeFallbacks, spriteIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForSpriteByUnicodeInternal_SpriteAsset1(
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
        unicode: u32,
        includeFallbacks: bool,
        spriteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
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
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                >,
            >,
        > = __cordl_object.invoke("get_spriteCharacterLookupTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spriteCharacterTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                >,
            >,
        > = __cordl_object.invoke("get_spriteCharacterTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spriteGlyphTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteGlyph,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteGlyph,
                >,
            >,
        > = __cordl_object.invoke("get_spriteGlyphTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spriteSheet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = __cordl_object
            .invoke("get_spriteSheet", ())?;
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
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                >,
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
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                >,
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
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteGlyph,
                >,
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
    pub fn set_spriteSheet(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteSheet", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::SpriteAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
