#[cfg(feature = "UnityEngine+SubsystemDescriptorBindings")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemDescriptorBindings {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+SubsystemDescriptorBindings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SubsystemDescriptorBindings =>
    "UnityEngine"."SubsystemDescriptorBindings"
);
#[cfg(feature = "UnityEngine+SubsystemDescriptorBindings")]
impl std::ops::Deref for crate::UnityEngine::SubsystemDescriptorBindings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemDescriptorBindings")]
impl std::ops::DerefMut for crate::UnityEngine::SubsystemDescriptorBindings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemDescriptorBindings")]
impl crate::UnityEngine::SubsystemDescriptorBindings {}
#[cfg(feature = "UnityEngine+SubsystemDescriptorBindings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SubsystemDescriptorBindings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
