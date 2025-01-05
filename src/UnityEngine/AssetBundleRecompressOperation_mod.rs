#[cfg(feature = "UnityEngine+AssetBundleRecompressOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetBundleRecompressOperation {
    __cordl_parent: crate::UnityEngine::AsyncOperation,
}
#[cfg(feature = "UnityEngine+AssetBundleRecompressOperation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AssetBundleRecompressOperation =>
    "UnityEngine"."AssetBundleRecompressOperation"
);
#[cfg(feature = "UnityEngine+AssetBundleRecompressOperation")]
impl std::ops::Deref for crate::UnityEngine::AssetBundleRecompressOperation {
    type Target = crate::UnityEngine::AsyncOperation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetBundleRecompressOperation")]
impl std::ops::DerefMut for crate::UnityEngine::AssetBundleRecompressOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetBundleRecompressOperation")]
impl crate::UnityEngine::AssetBundleRecompressOperation {}
#[cfg(feature = "UnityEngine+AssetBundleRecompressOperation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AssetBundleRecompressOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
