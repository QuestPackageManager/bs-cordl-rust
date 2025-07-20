#[cfg(feature = "UnityEngine+TextCore+Text+MaterialReferenceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialReferenceManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_FontMaterialReferenceLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
    >,
    pub m_FontAssetReferenceLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
        >,
    >,
    pub m_SpriteAssetReferenceLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteAsset>,
        >,
    >,
    pub m_ColorGradientReferenceLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextColorGradient,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialReferenceManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::MaterialReferenceManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "MaterialReferenceManager";
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
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialReferenceManager")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::MaterialReferenceManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialReferenceManager")]
impl std::ops::DerefMut
for crate::UnityEngine::TextCore::Text::MaterialReferenceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialReferenceManager")]
impl crate::UnityEngine::TextCore::Text::MaterialReferenceManager {
    pub fn AddColorGradientPreset(
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextColorGradient,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextColorGradient,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddColorGradientPreset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "AddColorGradientPreset",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (hashCode, spriteAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddColorGradientPreset_Internal(
        &mut self,
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextColorGradient,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextColorGradient,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddColorGradientPreset_Internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "AddColorGradientPreset_Internal", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (hashCode, spriteAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddFontAsset(
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::FontAsset,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddFontAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "AddFontAsset", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (fontAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddFontAssetInternal(
        &mut self,
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::FontAsset,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddFontAssetInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "AddFontAssetInternal",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (fontAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddFontMaterial(
        hashCode: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddFontMaterial")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "AddFontMaterial", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (hashCode, material))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddFontMaterialInternal(
        &mut self,
        hashCode: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddFontMaterialInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "AddFontMaterialInternal",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (hashCode, material))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddSpriteAsset(
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::SpriteAsset,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddSpriteAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "AddSpriteAsset", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (hashCode, spriteAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddSpriteAssetInternal(
        &mut self,
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::SpriteAsset,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddSpriteAssetInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "AddSpriteAssetInternal",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (hashCode, spriteAsset))?
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
    pub fn TryGetColorGradientPreset(
        hashCode: i32,
        gradientPreset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextColorGradient,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::TextColorGradient,
                        >,
                    >,
                ),
                bool,
                2usize,
            >("TryGetColorGradientPreset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "TryGetColorGradientPreset",
                    2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (hashCode, gradientPreset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetColorGradientPresetInternal(
        &mut self,
        hashCode: i32,
        gradientPreset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::TextCore::Text::TextColorGradient,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::TextColorGradient,
                        >,
                    >,
                ),
                bool,
                2usize,
            >("TryGetColorGradientPresetInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "TryGetColorGradientPresetInternal", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (hashCode, gradientPreset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetFontAsset(
        hashCode: i32,
        fontAsset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::FontAsset,
                        >,
                    >,
                ),
                bool,
                2usize,
            >("TryGetFontAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "TryGetFontAsset", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (hashCode, fontAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetFontAssetInternal(
        &mut self,
        hashCode: i32,
        fontAsset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::FontAsset,
                        >,
                    >,
                ),
                bool,
                2usize,
            >("TryGetFontAssetInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "TryGetFontAssetInternal",
                    2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (hashCode, fontAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetMaterial(
        hashCode: i32,
        material: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    >,
                ),
                bool,
                2usize,
            >("TryGetMaterial")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "TryGetMaterial", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (hashCode, material))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetMaterialInternal(
        &mut self,
        hashCode: i32,
        material: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    >,
                ),
                bool,
                2usize,
            >("TryGetMaterialInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "TryGetMaterialInternal",
                    2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (hashCode, material))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSpriteAsset(
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteAsset>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::SpriteAsset,
                        >,
                    >,
                ),
                bool,
                2usize,
            >("TryGetSpriteAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "TryGetSpriteAsset", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (hashCode, spriteAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSpriteAssetInternal(
        &mut self,
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::SpriteAsset>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::SpriteAsset,
                        >,
                    >,
                ),
                bool,
                2usize,
            >("TryGetSpriteAssetInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "TryGetSpriteAssetInternal",
                    2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (hashCode, spriteAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::MaterialReferenceManager,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::TextCore::Text::MaterialReferenceManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::MaterialReferenceManager,
                >,
                0usize,
            >("get_instance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::TextCore::Text::MaterialReferenceManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_instance", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::MaterialReferenceManager,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialReferenceManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::MaterialReferenceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
