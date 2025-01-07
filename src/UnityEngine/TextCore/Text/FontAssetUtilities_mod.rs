#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct FontAssetUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::FontAssetUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "FontAssetUtilities";
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
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::FontAssetUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::FontAssetUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
impl crate::UnityEngine::TextCore::Text::FontAssetUtilities {
    pub fn GetCharacterFromFontAsset(
        unicode: u32,
        sourceFontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
        includeFallbacks: bool,
        fontStyle: crate::UnityEngine::TextCore::Text::FontStyles,
        fontWeight: crate::UnityEngine::TextCore::Text::TextFontWeight,
        isAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::Character>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::Character,
        > = <Self as quest_hook::libil2cpp::Type>::class()
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
        sourceFontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
        includeFallbacks: bool,
        fontStyle: crate::UnityEngine::TextCore::Text::FontStyles,
        fontWeight: crate::UnityEngine::TextCore::Text::TextFontWeight,
        isAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::Character>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::Character,
        > = <Self as quest_hook::libil2cpp::Type>::class()
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
        sourceFontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
        fontAssets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
            >,
        >,
        includeFallbacks: bool,
        fontStyle: crate::UnityEngine::TextCore::Text::FontStyles,
        fontWeight: crate::UnityEngine::TextCore::Text::TextFontWeight,
        isAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::Character>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::Character,
        > = <Self as quest_hook::libil2cpp::Type>::class()
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
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
        includeFallbacks: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteCharacter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteCharacter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetSpriteCharacterFromSpriteAsset",
                (unicode, spriteAsset, includeFallbacks),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteCharacterFromSpriteAsset_Internal(
        unicode: u32,
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
        includeFallbacks: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteCharacter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteCharacter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetSpriteCharacterFromSpriteAsset_Internal",
                (unicode, spriteAsset, includeFallbacks),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::FontAssetUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
