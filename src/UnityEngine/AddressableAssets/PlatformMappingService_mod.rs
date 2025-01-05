#[cfg(feature = "UnityEngine+AddressableAssets+PlatformMappingService")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformMappingService {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+AddressableAssets+PlatformMappingService")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::PlatformMappingService =>
    "UnityEngine.AddressableAssets"."PlatformMappingService"
);
#[cfg(feature = "UnityEngine+AddressableAssets+PlatformMappingService")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::PlatformMappingService {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+PlatformMappingService")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::PlatformMappingService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+PlatformMappingService")]
impl crate::UnityEngine::AddressableAssets::PlatformMappingService {
    pub fn GetAddressablesPlatformInternal(
        platform: crate::UnityEngine::RuntimePlatform,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::AddressableAssets::AddressablesPlatform,
    > {
        let __cordl_ret: crate::UnityEngine::AddressableAssets::AddressablesPlatform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAddressablesPlatformInternal", (platform))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAddressablesPlatformPathInternal(
        platform: crate::UnityEngine::RuntimePlatform,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAddressablesPlatformPathInternal", (platform))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlatform() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::AddressableAssets::AddressablesPlatform,
    > {
        let __cordl_ret: crate::UnityEngine::AddressableAssets::AddressablesPlatform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlatform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlatformPathSubFolder() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlatformPathSubFolder", ())?;
        Ok(__cordl_ret.into())
    }
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
}
#[cfg(feature = "UnityEngine+AddressableAssets+PlatformMappingService")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::PlatformMappingService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
