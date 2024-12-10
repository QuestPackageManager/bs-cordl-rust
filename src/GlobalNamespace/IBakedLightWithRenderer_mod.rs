#[cfg(feature = "IBakedLightWithRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct IBakedLightWithRenderer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IBakedLightWithRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IBakedLightWithRenderer => ""
    ."IBakedLightWithRenderer"
);
#[cfg(feature = "IBakedLightWithRenderer")]
impl std::ops::Deref for crate::GlobalNamespace::IBakedLightWithRenderer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBakedLightWithRenderer")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBakedLightWithRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBakedLightWithRenderer")]
impl crate::GlobalNamespace::IBakedLightWithRenderer {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_bakingMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_bakingMaterial", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IBakedLightWithRenderer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IBakedLightWithRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
