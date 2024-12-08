#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+JsonAssetProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonAssetProvider {
    __cordl_parent: crate::UnityEngine::ResourceManagement::ResourceProviders::TextDataProvider,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+JsonAssetProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::JsonAssetProvider =>
    "UnityEngine.ResourceManagement.ResourceProviders"."JsonAssetProvider"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+JsonAssetProvider")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::JsonAssetProvider {
    type Target = crate::UnityEngine::ResourceManagement::ResourceProviders::TextDataProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+JsonAssetProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::JsonAssetProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+JsonAssetProvider")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::JsonAssetProvider {
    pub fn Convert(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Convert", (_cordl_type, text))?;
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+JsonAssetProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::JsonAssetProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
