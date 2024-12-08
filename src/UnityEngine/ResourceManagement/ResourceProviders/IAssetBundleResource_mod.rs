#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
#[repr(C)]
#[derive(Debug)]
pub struct IAssetBundleResource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource =>
    "UnityEngine.ResourceManagement.ResourceProviders"."IAssetBundleResource"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
    pub fn GetAssetBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AssetBundle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AssetBundle = __cordl_object
            .invoke("GetAssetBundle", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
