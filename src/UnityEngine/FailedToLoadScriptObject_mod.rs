#[cfg(feature = "UnityEngine+FailedToLoadScriptObject")]
#[repr(C)]
#[derive(Debug)]
pub struct FailedToLoadScriptObject {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+FailedToLoadScriptObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FailedToLoadScriptObject =>
    "UnityEngine"."FailedToLoadScriptObject"
);
#[cfg(feature = "UnityEngine+FailedToLoadScriptObject")]
impl std::ops::Deref for crate::UnityEngine::FailedToLoadScriptObject {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+FailedToLoadScriptObject")]
impl std::ops::DerefMut for crate::UnityEngine::FailedToLoadScriptObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+FailedToLoadScriptObject")]
impl crate::UnityEngine::FailedToLoadScriptObject {}
#[cfg(feature = "UnityEngine+FailedToLoadScriptObject")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::FailedToLoadScriptObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
