#[cfg(feature = "UnityEngine+AssetBundleCreateRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetBundleCreateRequest {
    __cordl_parent: crate::UnityEngine::AsyncOperation,
}
#[cfg(feature = "UnityEngine+AssetBundleCreateRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AssetBundleCreateRequest =>
    "UnityEngine"."AssetBundleCreateRequest"
);
#[cfg(feature = "UnityEngine+AssetBundleCreateRequest")]
impl std::ops::Deref for crate::UnityEngine::AssetBundleCreateRequest {
    type Target = crate::UnityEngine::AsyncOperation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetBundleCreateRequest")]
impl std::ops::DerefMut for crate::UnityEngine::AssetBundleCreateRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetBundleCreateRequest")]
impl crate::UnityEngine::AssetBundleCreateRequest {
    pub fn get_assetBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AssetBundle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AssetBundle = __cordl_object
            .invoke("get_assetBundle", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+AssetBundleCreateRequest")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AssetBundleCreateRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
