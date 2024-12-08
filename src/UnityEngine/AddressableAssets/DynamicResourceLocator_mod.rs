#[cfg(feature = "UnityEngine+AddressableAssets+DynamicResourceLocator")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicResourceLocator {
    __cordl_parent: crate::System::Object,
    pub m_Addressables: *mut crate::UnityEngine::AddressableAssets::AddressablesImpl,
    pub m_AtlasSpriteProviderId: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+AddressableAssets+DynamicResourceLocator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::DynamicResourceLocator =>
    "UnityEngine.AddressableAssets"."DynamicResourceLocator"
);
#[cfg(feature = "UnityEngine+AddressableAssets+DynamicResourceLocator")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::DynamicResourceLocator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+DynamicResourceLocator")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::DynamicResourceLocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+DynamicResourceLocator")]
impl crate::UnityEngine::AddressableAssets::DynamicResourceLocator {
    pub fn _ctor(
        &mut self,
        addr: *mut crate::UnityEngine::AddressableAssets::AddressablesImpl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (addr))?;
        Ok(__cordl_ret)
    }
    pub fn get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_Keys", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AtlasSpriteProviderId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AtlasSpriteProviderId", ())?;
        Ok(__cordl_ret)
    }
    pub fn Locate(
        &mut self,
        key: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
        locations: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Locate", (key, _cordl_type, locations))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDynamicLocations(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        locName: *mut crate::System::String,
        subKey: *mut crate::System::String,
        mainLoc: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CreateDynamicLocations",
                (_cordl_type, locations, locName, subKey, mainLoc),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_AllLocations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        > = __cordl_object.invoke("get_AllLocations", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocatorId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LocatorId", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        addr: *mut crate::UnityEngine::AddressableAssets::AddressablesImpl,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (addr))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+DynamicResourceLocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::DynamicResourceLocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
