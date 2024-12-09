#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemDescriptorStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SubsystemsImplementation::SubsystemDescriptorStore =>
    "UnityEngine.SubsystemsImplementation"."SubsystemDescriptorStore"
);
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
impl std::ops::Deref
for crate::UnityEngine::SubsystemsImplementation::SubsystemDescriptorStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
impl std::ops::DerefMut
for crate::UnityEngine::SubsystemsImplementation::SubsystemDescriptorStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
impl crate::UnityEngine::SubsystemsImplementation::SubsystemDescriptorStore {}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SubsystemsImplementation::SubsystemDescriptorStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
