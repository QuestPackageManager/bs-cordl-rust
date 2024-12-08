#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct WebRequestQueue {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ResourceManagement::WebRequestQueue
    => "UnityEngine.ResourceManagement"."WebRequestQueue"
);
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
impl std::ops::Deref for crate::UnityEngine::ResourceManagement::WebRequestQueue {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
impl std::ops::DerefMut for crate::UnityEngine::ResourceManagement::WebRequestQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
impl crate::UnityEngine::ResourceManagement::WebRequestQueue {}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::WebRequestQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
