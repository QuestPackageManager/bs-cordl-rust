#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+SpriteAsset")]
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+SpriteAsset")]
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::SpriteAsset {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+SpriteAsset")]
impl crate::UnityEngine::TextCore::Text::SpriteAsset {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Awake",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteIndexFromHashcode(
        &mut self,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("GetSpriteIndexFromHashcode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSpriteIndexFromHashcode", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (hashCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteIndexFromName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        i32,
                        1usize,
                    >("GetSpriteIndexFromName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSpriteIndexFromName", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteIndexFromUnicode(
        &mut self,
        unicode: u32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), i32, 1usize>("GetSpriteIndexFromUnicode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSpriteIndexFromUnicode", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (unicode))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::SpriteAsset,
                            >,
                            i32,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextSettings,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::SpriteAsset,
                        >,
                        5usize,
                    >("SearchForSpriteByHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SearchForSpriteByHashCode", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (spriteAsset, hashCode, includeFallbacks, spriteIndex, textSettings),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::TextCore::Text::SpriteAsset,
                                    >,
                                >,
                            >,
                            i32,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::SpriteAsset,
                        >,
                        4usize,
                    >("SearchForSpriteByHashCodeInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SearchForSpriteByHashCodeInternal", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (spriteAssets, hashCode, searchFallbacks, spriteIndex),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::SpriteAsset,
                            >,
                            i32,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::SpriteAsset,
                        >,
                        4usize,
                    >("SearchForSpriteByHashCodeInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SearchForSpriteByHashCodeInternal", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (spriteAsset, hashCode, searchFallbacks, spriteIndex),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::SpriteAsset,
                            >,
                            u32,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::SpriteAsset,
                        >,
                        4usize,
                    >("SearchForSpriteByUnicode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SearchForSpriteByUnicode", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (spriteAsset, unicode, includeFallbacks, spriteIndex),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::TextCore::Text::SpriteAsset,
                                    >,
                                >,
                            >,
                            u32,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::SpriteAsset,
                        >,
                        4usize,
                    >("SearchForSpriteByUnicodeInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SearchForSpriteByUnicodeInternal", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (spriteAssets, unicode, includeFallbacks, spriteIndex),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::SpriteAsset,
                            >,
                            u32,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::SpriteAsset,
                        >,
                        4usize,
                    >("SearchForSpriteByUnicodeInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SearchForSpriteByUnicodeInternal", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (spriteAsset, unicode, includeFallbacks, spriteIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SortCharacterTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("SortCharacterTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SortCharacterTable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SortGlyphAndCharacterTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("SortGlyphAndCharacterTables")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SortGlyphAndCharacterTables", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SortGlyphTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("SortGlyphTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SortGlyphTable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLookupTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("UpdateLookupTables")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateLookupTables", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_faceInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::FaceInfo> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::TextCore::FaceInfo,
                        0usize,
                    >("get_faceInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_faceInfo", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::FaceInfo = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                u32,
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_spriteCharacterLookupTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_spriteCharacterLookupTable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_spriteCharacterTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_spriteCharacterTable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::SpriteGlyph,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_spriteGlyphTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_spriteGlyphTable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::SpriteGlyph,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_spriteSheet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        0usize,
                    >("get_spriteSheet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_spriteSheet", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_faceInfo(
        &mut self,
        value: crate::UnityEngine::TextCore::FaceInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::TextCore::FaceInfo),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_faceInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_faceInfo", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                u32,
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_spriteCharacterLookupTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_spriteCharacterLookupTable", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::SpriteCharacter,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_spriteCharacterTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_spriteCharacterTable", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Text::SpriteGlyph,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_spriteGlyphTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_spriteGlyphTable", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_spriteSheet(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_spriteSheet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_spriteSheet", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+SpriteAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::SpriteAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
