#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_FontAssetUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_FontAssetUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_FontAssetUtilities";
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
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_FontAssetUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
impl std::ops::DerefMut for crate::TMPro::TMP_FontAssetUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                            bool,
                            crate::TMPro::FontStyles,
                            crate::TMPro::FontWeight,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
                        6usize,
                    >("GetCharacterFromFontAsset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCharacterFromFontAsset", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character> = unsafe {
            method
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
        sourceFontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        includeFallbacks: bool,
        fontStyle: crate::TMPro::FontStyles,
        fontWeight: crate::TMPro::FontWeight,
        isAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                            bool,
                            crate::TMPro::FontStyles,
                            crate::TMPro::FontWeight,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
                        6usize,
                    >("GetCharacterFromFontAsset_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCharacterFromFontAsset_Internal", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character> = unsafe {
            method
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
        sourceFontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        fontAssets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
            >,
        >,
        includeFallbacks: bool,
        fontStyle: crate::TMPro::FontStyles,
        fontWeight: crate::TMPro::FontWeight,
        isAlternativeTypeface: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                                >,
                            >,
                            bool,
                            crate::TMPro::FontStyles,
                            crate::TMPro::FontWeight,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
                        7usize,
                    >("GetCharacterFromFontAssets")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCharacterFromFontAssets", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character> = unsafe {
            method
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
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        includeFallbacks: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteCharacter>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteCharacter>,
                        3usize,
                    >("GetSpriteCharacterFromSpriteAsset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSpriteCharacterFromSpriteAsset", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteCharacter> = unsafe {
            method.invoke_unchecked((), (unicode, spriteAsset, includeFallbacks))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteCharacterFromSpriteAsset_Internal(
        unicode: u32,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        includeFallbacks: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteCharacter>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteCharacter>,
                        3usize,
                    >("GetSpriteCharacterFromSpriteAsset_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSpriteCharacterFromSpriteAsset_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteCharacter> = unsafe {
            method.invoke_unchecked((), (unicode, spriteAsset, includeFallbacks))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAssetUtilities>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAssetUtilities>,
                        0usize,
                    >("get_instance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_instance", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::TMPro::TMP_FontAssetUtilities,
        > = unsafe { method.invoke_unchecked((), ())? };
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
