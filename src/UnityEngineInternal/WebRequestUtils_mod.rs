#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct WebRequestUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngineInternal::WebRequestUtils =>
    "UnityEngineInternal"."WebRequestUtils"
);
#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
impl std::ops::Deref for crate::UnityEngineInternal::WebRequestUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
impl std::ops::DerefMut for crate::UnityEngineInternal::WebRequestUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
impl crate::UnityEngineInternal::WebRequestUtils {}
#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngineInternal::WebRequestUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
