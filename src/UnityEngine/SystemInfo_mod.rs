#[cfg(feature = "UnityEngine+SystemInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemInfo {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+SystemInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SystemInfo => "UnityEngine"
    ."SystemInfo"
);
#[cfg(feature = "UnityEngine+SystemInfo")]
impl std::ops::Deref for crate::UnityEngine::SystemInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SystemInfo")]
impl std::ops::DerefMut for crate::UnityEngine::SystemInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SystemInfo")]
impl crate::UnityEngine::SystemInfo {}
#[cfg(feature = "UnityEngine+SystemInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SystemInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}