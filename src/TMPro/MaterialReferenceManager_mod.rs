#[cfg(feature = "TMPro+MaterialReferenceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialReferenceManager {
    __cordl_parent: crate::System::Object,
    pub m_FontMaterialReferenceLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::UnityEngine::Material,
    >,
    pub m_FontAssetReferenceLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::TMPro::TMP_FontAsset,
    >,
    pub m_SpriteAssetReferenceLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::TMPro::TMP_SpriteAsset,
    >,
    pub m_ColorGradientReferenceLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::TMPro::TMP_ColorGradient,
    >,
}
#[cfg(feature = "TMPro+MaterialReferenceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::MaterialReferenceManager => "TMPro"
    ."MaterialReferenceManager"
);
#[cfg(feature = "TMPro+MaterialReferenceManager")]
impl std::ops::Deref for crate::TMPro::MaterialReferenceManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+MaterialReferenceManager")]
impl std::ops::DerefMut for crate::TMPro::MaterialReferenceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+MaterialReferenceManager")]
impl crate::TMPro::MaterialReferenceManager {
    pub fn AddColorGradientPreset_Internal(
        &mut self,
        hashCode: i32,
        spriteAsset: *mut crate::TMPro::TMP_ColorGradient,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddColorGradientPreset_Internal", (hashCode, spriteAsset))?;
        Ok(__cordl_ret)
    }
    pub fn AddFontAssetInternal(
        &mut self,
        fontAsset: *mut crate::TMPro::TMP_FontAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFontAssetInternal", (fontAsset))?;
        Ok(__cordl_ret)
    }
    pub fn AddFontMaterialInternal(
        &mut self,
        hashCode: i32,
        material: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFontMaterialInternal", (hashCode, material))?;
        Ok(__cordl_ret)
    }
    pub fn AddSpriteAssetInternal_TMP_SpriteAsset0(
        &mut self,
        spriteAsset: *mut crate::TMPro::TMP_SpriteAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSpriteAssetInternal", (spriteAsset))?;
        Ok(__cordl_ret)
    }
    pub fn AddSpriteAssetInternal_i32_TMP_SpriteAsset1(
        &mut self,
        hashCode: i32,
        spriteAsset: *mut crate::TMPro::TMP_SpriteAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSpriteAssetInternal", (hashCode, spriteAsset))?;
        Ok(__cordl_ret)
    }
    pub fn Contains_TMP_FontAsset0(
        &mut self,
        font: *mut crate::TMPro::TMP_FontAsset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (font))?;
        Ok(__cordl_ret)
    }
    pub fn Contains_TMP_SpriteAsset1(
        &mut self,
        sprite: *mut crate::TMPro::TMP_SpriteAsset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (sprite))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn TryGetColorGradientPresetInternal(
        &mut self,
        hashCode: i32,
        gradientPreset: quest_hook::libil2cpp::ByRefMut<
            *mut crate::TMPro::TMP_ColorGradient,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetColorGradientPresetInternal", (hashCode, gradientPreset))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetFontAssetInternal(
        &mut self,
        hashCode: i32,
        fontAsset: quest_hook::libil2cpp::ByRefMut<*mut crate::TMPro::TMP_FontAsset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetFontAssetInternal", (hashCode, fontAsset))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetMaterialInternal(
        &mut self,
        hashCode: i32,
        material: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetMaterialInternal", (hashCode, material))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetSpriteAssetInternal(
        &mut self,
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::ByRefMut<*mut crate::TMPro::TMP_SpriteAsset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetSpriteAssetInternal", (hashCode, spriteAsset))?;
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
}
#[cfg(feature = "TMPro+MaterialReferenceManager")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::MaterialReferenceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
