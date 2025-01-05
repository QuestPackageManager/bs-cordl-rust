#[cfg(feature = "UnityEngine+AssetBundleUnloadOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetBundleUnloadOperation {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
}
#[cfg(feature = "UnityEngine+AssetBundleUnloadOperation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AssetBundleUnloadOperation =>
    "UnityEngine"."AssetBundleUnloadOperation"
);
#[cfg(feature = "UnityEngine+AssetBundleUnloadOperation")]
impl std::ops::Deref for crate::UnityEngine::AssetBundleUnloadOperation {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetBundleUnloadOperation")]
impl std::ops::DerefMut for crate::UnityEngine::AssetBundleUnloadOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetBundleUnloadOperation")]
impl crate::UnityEngine::AssetBundleUnloadOperation {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn WaitForCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WaitForCompletion", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+AssetBundleUnloadOperation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AssetBundleUnloadOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
