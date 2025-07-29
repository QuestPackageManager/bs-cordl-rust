#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+FontAssetUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct FontAssetUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+FontAssetUtilities")]
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::FontAssetUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::FontAsset,
                            >,
                            bool,
                            crate::UnityEngine::TextCore::Text::FontStyles,
                            crate::UnityEngine::TextCore::Text::TextFontWeight,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::Character,
                        >,
                        6usize,
                    >("GetCharacterFromFontAsset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCharacterFromFontAsset", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::Character,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        unicode,
                        sourceFontAsset,
                        includeFallbacks,
                        fontStyle,
                        fontWeight,
                        isAlternativeTypeface,
                    ),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::FontAsset,
                            >,
                            bool,
                            crate::UnityEngine::TextCore::Text::FontStyles,
                            crate::UnityEngine::TextCore::Text::TextFontWeight,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::Character,
                        >,
                        6usize,
                    >("GetCharacterFromFontAsset_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCharacterFromFontAsset_Internal", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::Character,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        unicode,
                        sourceFontAsset,
                        includeFallbacks,
                        fontStyle,
                        fontWeight,
                        isAlternativeTypeface,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCharacterFromFontAssets(
        unicode: u32,
        sourceFontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
        fontAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
        >,
        includeFallbacks: bool,
        fontStyle: crate::UnityEngine::TextCore::Text::FontStyles,
        fontWeight: crate::UnityEngine::TextCore::Text::TextFontWeight,
        isAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::Character>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::FontAsset,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::FontAsset,
                                >,
                            >,
                            bool,
                            crate::UnityEngine::TextCore::Text::FontStyles,
                            crate::UnityEngine::TextCore::Text::TextFontWeight,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::Character,
                        >,
                        7usize,
                    >("GetCharacterFromFontAssets")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCharacterFromFontAssets", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::Character,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        unicode,
                        sourceFontAsset,
                        fontAssets,
                        includeFallbacks,
                        fontStyle,
                        fontWeight,
                        isAlternativeTypeface,
                    ),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::SpriteAsset,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::SpriteCharacter,
                        >,
                        3usize,
                    >("GetSpriteCharacterFromSpriteAsset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSpriteCharacterFromSpriteAsset", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteCharacter,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (unicode, spriteAsset, includeFallbacks))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::SpriteAsset,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::SpriteCharacter,
                        >,
                        3usize,
                    >("GetSpriteCharacterFromSpriteAsset_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSpriteCharacterFromSpriteAsset_Internal", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteCharacter,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (unicode, spriteAsset, includeFallbacks))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+FontAssetUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::FontAssetUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
