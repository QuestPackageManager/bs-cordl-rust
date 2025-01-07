#[cfg(feature = "UnityEngine+TextCore+Text+MaterialManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::MaterialManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "MaterialManager";
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
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialManager")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::MaterialManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialManager")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::MaterialManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialManager")]
impl crate::UnityEngine::TextCore::Text::MaterialManager {
    pub fn CopyMaterialPresetProperties(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyMaterialPresetProperties", (source, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFallbackMaterial_FontAsset_i32_1(
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
        sourceMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        atlasIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFallbackMaterial", (fontAsset, sourceMaterial, atlasIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFallbackMaterial_Material0(
        sourceMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        targetMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFallbackMaterial", (sourceMaterial, targetMaterial))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+MaterialManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::MaterialManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
