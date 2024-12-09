#[cfg(feature = "Zenject+PrefabProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _prefab: *mut crate::UnityEngine::Object,
}
#[cfg(feature = "Zenject+PrefabProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PrefabProvider => "Zenject"
    ."PrefabProvider"
);
#[cfg(feature = "Zenject+PrefabProvider")]
impl std::ops::Deref for crate::Zenject::PrefabProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabProvider")]
impl std::ops::DerefMut for crate::Zenject::PrefabProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabProvider")]
impl crate::Zenject::PrefabProvider {
    pub fn GetPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("GetPrefab", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (prefab))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (prefab))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+PrefabProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::PrefabProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
