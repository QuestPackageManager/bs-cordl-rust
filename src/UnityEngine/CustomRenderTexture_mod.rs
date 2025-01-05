#[cfg(feature = "UnityEngine+CustomRenderTexture")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomRenderTexture {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
}
#[cfg(feature = "UnityEngine+CustomRenderTexture")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CustomRenderTexture =>
    "UnityEngine"."CustomRenderTexture"
);
#[cfg(feature = "UnityEngine+CustomRenderTexture")]
impl std::ops::Deref for crate::UnityEngine::CustomRenderTexture {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CustomRenderTexture")]
impl std::ops::DerefMut for crate::UnityEngine::CustomRenderTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CustomRenderTexture")]
impl crate::UnityEngine::CustomRenderTexture {}
#[cfg(feature = "UnityEngine+CustomRenderTexture")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::CustomRenderTexture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
