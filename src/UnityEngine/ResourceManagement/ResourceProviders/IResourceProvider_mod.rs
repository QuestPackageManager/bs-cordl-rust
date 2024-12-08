#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IResourceProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IResourceProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IResourceProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider =>
    "UnityEngine.ResourceManagement.ResourceProviders"."IResourceProvider"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IResourceProvider")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IResourceProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IResourceProvider")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider {
    pub fn get_ProviderId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ProviderId", ())?;
        Ok(__cordl_ret)
    }
    pub fn CanProvide(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanProvide", (_cordl_type, location))?;
        Ok(__cordl_ret)
    }
    pub fn Provide(
        &mut self,
        provideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Provide", (provideHandle))?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        asset: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", (location, asset))?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultType(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetDefaultType", (location))?;
        Ok(__cordl_ret)
    }
    pub fn get_BehaviourFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::ResourceProviders::ProviderBehaviourFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::ResourceProviders::ProviderBehaviourFlags = __cordl_object
            .invoke("get_BehaviourFlags", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IResourceProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
