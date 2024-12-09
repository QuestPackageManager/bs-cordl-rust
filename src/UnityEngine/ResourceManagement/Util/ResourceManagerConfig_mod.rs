#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManagerConfig {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::ResourceManagerConfig =>
    "UnityEngine.ResourceManagement.Util"."ResourceManagerConfig"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::ResourceManagerConfig {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::ResourceManagerConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
impl crate::UnityEngine::ResourceManagement::Util::ResourceManagerConfig {}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::ResourceManagerConfig {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
