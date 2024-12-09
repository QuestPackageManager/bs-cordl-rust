#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct HashCodeHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::HashCodeHelper =>
    "UnityEngine.XR"."HashCodeHelper"
);
#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
impl std::ops::Deref for crate::UnityEngine::XR::HashCodeHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
impl std::ops::DerefMut for crate::UnityEngine::XR::HashCodeHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
impl crate::UnityEngine::XR::HashCodeHelper {}
#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::HashCodeHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
