#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_FontAssetUtilities {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_FontAssetUtilities => "TMPro"
    ."TMP_FontAssetUtilities"
);
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_FontAssetUtilities {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
impl std::ops::DerefMut for crate::TMPro::TMP_FontAssetUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
impl crate::TMPro::TMP_FontAssetUtilities {
    pub fn GetCharacterFromFontAsset(
        unicode: u32,
        sourceFontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        includeFallbacks: bool,
        fontStyle: crate::TMPro::FontStyles,
        fontWeight: crate::TMPro::FontWeight,
        isAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetCharacterFromFontAsset",
                (
                    unicode,
                    sourceFontAsset,
                    includeFallbacks,
                    fontStyle,
                    fontWeight,
                    isAlternativeTypeface,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCharacterFromFontAsset_Internal(
        unicode: u32,
        sourceFontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        includeFallbacks: bool,
        fontStyle: crate::TMPro::FontStyles,
        fontWeight: crate::TMPro::FontWeight,
        isAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetCharacterFromFontAsset_Internal",
                (
                    unicode,
                    sourceFontAsset,
                    includeFallbacks,
                    fontStyle,
                    fontWeight,
                    isAlternativeTypeface,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCharacterFromFontAssets(
        unicode: u32,
        sourceFontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        fontAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        >,
        includeFallbacks: bool,
        fontStyle: crate::TMPro::FontStyles,
        fontWeight: crate::TMPro::FontWeight,
        isAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetCharacterFromFontAssets",
                (
                    unicode,
                    sourceFontAsset,
                    fontAssets,
                    includeFallbacks,
                    fontStyle,
                    fontWeight,
                    isAlternativeTypeface,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteCharacterFromSpriteAsset(
        unicode: u32,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        includeFallbacks: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteCharacter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteCharacter> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetSpriteCharacterFromSpriteAsset",
                (unicode, spriteAsset, includeFallbacks),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteCharacterFromSpriteAsset_Internal(
        unicode: u32,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        includeFallbacks: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteCharacter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteCharacter> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetSpriteCharacterFromSpriteAsset_Internal",
                (unicode, spriteAsset, includeFallbacks),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAssetUtilities>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::TMPro::TMP_FontAssetUtilities,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_FontAssetUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
