#[cfg(feature = "Zenject+PrefabProviderResource")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabProviderResource {
    __cordl_parent: crate::System::Object,
    pub _resourcePath: *mut crate::System::String,
}
#[cfg(feature = "Zenject+PrefabProviderResource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PrefabProviderResource => "Zenject"
    ."PrefabProviderResource"
);
#[cfg(feature = "Zenject+PrefabProviderResource")]
impl std::ops::Deref for crate::Zenject::PrefabProviderResource {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabProviderResource")]
impl std::ops::DerefMut for crate::Zenject::PrefabProviderResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabProviderResource")]
impl crate::Zenject::PrefabProviderResource {
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
    pub fn _ctor(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resourcePath))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+PrefabProviderResource")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::PrefabProviderResource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
