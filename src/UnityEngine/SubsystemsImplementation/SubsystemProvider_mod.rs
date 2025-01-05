#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemProvider {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_Running: bool,
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SubsystemsImplementation::SubsystemProvider =>
    "UnityEngine.SubsystemsImplementation"."SubsystemProvider"
);
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemProvider")]
impl std::ops::Deref
for crate::UnityEngine::SubsystemsImplementation::SubsystemProvider {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::SubsystemsImplementation::SubsystemProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemProvider")]
impl crate::UnityEngine::SubsystemsImplementation::SubsystemProvider {}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SubsystemsImplementation::SubsystemProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
