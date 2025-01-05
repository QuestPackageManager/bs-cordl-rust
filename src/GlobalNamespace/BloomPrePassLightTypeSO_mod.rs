#[cfg(feature = "BloomPrePassLightTypeSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassLightTypeSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _renderingPriority: i32,
    pub _material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "BloomPrePassLightTypeSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomPrePassLightTypeSO => ""
    ."BloomPrePassLightTypeSO"
);
#[cfg(feature = "BloomPrePassLightTypeSO")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassLightTypeSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassLightTypeSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomPrePassLightTypeSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassLightTypeSO")]
impl crate::GlobalNamespace::BloomPrePassLightTypeSO {
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
    pub fn get_material(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_material", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderingPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_renderingPriority", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassLightTypeSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassLightTypeSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
