#[cfg(feature = "AddressablesInternalBridge+Runtime+CatalogLocationData")]
#[repr(C)]
#[derive(Debug)]
pub struct CatalogLocationData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _LocatorId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _LocalHash_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _CatalogLocation_k__BackingField: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+CatalogLocationData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::AddressablesInternalBridge::Runtime::CatalogLocationData =>
    "AddressablesInternalBridge.Runtime"."CatalogLocationData"
);
#[cfg(feature = "AddressablesInternalBridge+Runtime+CatalogLocationData")]
impl std::ops::Deref
for crate::AddressablesInternalBridge::Runtime::CatalogLocationData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+CatalogLocationData")]
impl std::ops::DerefMut
for crate::AddressablesInternalBridge::Runtime::CatalogLocationData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+CatalogLocationData")]
impl crate::AddressablesInternalBridge::Runtime::CatalogLocationData {
    pub fn New(
        locatorId: *mut quest_hook::libil2cpp::Il2CppString,
        localHash: *mut quest_hook::libil2cpp::Il2CppString,
        resourceLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (locatorId, localHash, resourceLocation))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        locatorId: *mut quest_hook::libil2cpp::Il2CppString,
        localHash: *mut quest_hook::libil2cpp::Il2CppString,
        resourceLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (locatorId, localHash, resourceLocation))?;
        Ok(__cordl_ret)
    }
    pub fn get_CatalogLocation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation = __cordl_object
            .invoke("get_CatalogLocation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_LocalHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocatorId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_LocatorId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+CatalogLocationData")]
impl quest_hook::libil2cpp::ObjectType
for crate::AddressablesInternalBridge::Runtime::CatalogLocationData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
