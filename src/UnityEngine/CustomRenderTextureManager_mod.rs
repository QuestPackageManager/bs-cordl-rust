#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomRenderTextureManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CustomRenderTextureManager =>
    "UnityEngine"."CustomRenderTextureManager"
);
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
impl std::ops::Deref for crate::UnityEngine::CustomRenderTextureManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
impl std::ops::DerefMut for crate::UnityEngine::CustomRenderTextureManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
impl crate::UnityEngine::CustomRenderTextureManager {
    pub fn InvokeOnTextureLoaded_Internal(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::CustomRenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeOnTextureLoaded_Internal", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnTextureUnloaded_Internal(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::CustomRenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeOnTextureUnloaded_Internal", (source))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::CustomRenderTextureManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
