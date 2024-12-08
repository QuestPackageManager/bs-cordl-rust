#[cfg(feature = "UnityEngine+SubsystemManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemManager {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+SubsystemManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SubsystemManager => "UnityEngine"
    ."SubsystemManager"
);
#[cfg(feature = "UnityEngine+SubsystemManager")]
impl std::ops::Deref for crate::UnityEngine::SubsystemManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemManager")]
impl std::ops::DerefMut for crate::UnityEngine::SubsystemManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemManager")]
impl crate::UnityEngine::SubsystemManager {}
#[cfg(feature = "UnityEngine+SubsystemManager")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SubsystemManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
