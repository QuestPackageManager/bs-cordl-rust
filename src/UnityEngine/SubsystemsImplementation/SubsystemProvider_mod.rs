#[cfg(feature = "cordl_class_UnityEngine+SubsystemsImplementation+SubsystemProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Running: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+SubsystemsImplementation+SubsystemProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::SubsystemsImplementation::SubsystemProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.SubsystemsImplementation";
    const CLASS_NAME: &'static str = "SubsystemProvider";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "cordl_class_UnityEngine+SubsystemsImplementation+SubsystemProvider")]
impl std::ops::Deref
for crate::UnityEngine::SubsystemsImplementation::SubsystemProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+SubsystemsImplementation+SubsystemProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::SubsystemsImplementation::SubsystemProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemProvider")]
impl crate::UnityEngine::SubsystemsImplementation::SubsystemProvider {}
#[cfg(feature = "cordl_class_UnityEngine+SubsystemsImplementation+SubsystemProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SubsystemsImplementation::SubsystemProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
