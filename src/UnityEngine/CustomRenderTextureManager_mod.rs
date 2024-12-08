#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomRenderTextureManager {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CustomRenderTextureManager =>
    "UnityEngine"."CustomRenderTextureManager"
);
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
impl std::ops::Deref for crate::UnityEngine::CustomRenderTextureManager {
    type Target = crate::System::Object;
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
impl crate::UnityEngine::CustomRenderTextureManager {}
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
