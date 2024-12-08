#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+IResourceLocator")]
#[repr(C)]
#[derive(Debug)]
pub struct IResourceLocator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+IResourceLocator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator =>
    "UnityEngine.AddressableAssets.ResourceLocators"."IResourceLocator"
);
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+IResourceLocator")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+IResourceLocator")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+IResourceLocator")]
impl crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+IResourceLocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
