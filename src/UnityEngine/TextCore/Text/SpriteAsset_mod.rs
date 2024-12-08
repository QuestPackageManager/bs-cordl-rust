#[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteAsset {
    __cordl_parent: crate::UnityEngine::TextCore::Text::TextAsset,
    pub m_NameLookup: *mut crate::System::Collections::Generic::Dictionary_2<i32, i32>,
    pub m_GlyphIndexLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        i32,
    >,
    pub m_FaceInfo: crate::UnityEngine::TextCore::FaceInfo,
    pub m_SpriteAtlasTexture: *mut crate::UnityEngine::Texture,
    pub m_SpriteCharacterTable: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Text::SpriteCharacter,
    >,
    pub m_SpriteCharacterLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        *mut crate::UnityEngine::TextCore::Text::SpriteCharacter,
    >,
    pub m_SpriteGlyphTable: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Text::SpriteGlyph,
    >,
    pub m_SpriteGlyphLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        *mut crate::UnityEngine::TextCore::Text::SpriteGlyph,
    >,
    pub fallbackSpriteAssets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
    >,
    pub m_IsSpriteAssetLookupTablesDirty: bool,
}
#[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::SpriteAsset =>
    "UnityEngine.TextCore.Text"."SpriteAsset"
);
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
    #[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset+__c")]
    pub type __c = crate::UnityEngine::TextCore::Text::SpriteAsset___c;
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
    pub fn SortGlyphAndCharacterTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortGlyphAndCharacterTables", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_spriteCharacterTable(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::SpriteCharacter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteCharacterTable", (value))?;
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
    pub fn get_spriteSheet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture = __cordl_object
            .invoke("get_spriteSheet", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_spriteCharacterLookupTable(
        &mut self,
        value: *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::UnityEngine::TextCore::Text::SpriteCharacter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteCharacterLookupTable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_spriteGlyphTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::SpriteGlyph,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::SpriteGlyph,
        > = __cordl_object.invoke("get_spriteGlyphTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateLookupTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLookupTables", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_spriteCharacterTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::SpriteCharacter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::SpriteCharacter,
        > = __cordl_object.invoke("get_spriteCharacterTable", ())?;
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
    pub fn set_spriteSheet(
        &mut self,
        value: *mut crate::UnityEngine::Texture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteSheet", (value))?;
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
    pub fn GetSpriteIndexFromUnicode(
        &mut self,
        unicode: u32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetSpriteIndexFromUnicode", (unicode))?;
        Ok(__cordl_ret)
    }
    pub fn GetSpriteIndexFromName(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSpriteIndexFromName", (name))?;
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
    pub fn get_spriteCharacterLookupTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::UnityEngine::TextCore::Text::SpriteCharacter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::UnityEngine::TextCore::Text::SpriteCharacter,
        > = __cordl_object.invoke("get_spriteCharacterLookupTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_spriteGlyphTable(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::SpriteGlyph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteGlyphTable", (value))?;
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
