#[cfg(feature = "UnityEngine+Internal_SubsystemDescriptors")]
#[repr(C)]
#[derive(Debug)]
pub struct Internal_SubsystemDescriptors {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Internal_SubsystemDescriptors")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Internal_SubsystemDescriptors {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Internal_SubsystemDescriptors";
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
#[cfg(feature = "UnityEngine+Internal_SubsystemDescriptors")]
impl std::ops::Deref for crate::UnityEngine::Internal_SubsystemDescriptors {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Internal_SubsystemDescriptors")]
impl std::ops::DerefMut for crate::UnityEngine::Internal_SubsystemDescriptors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Internal_SubsystemDescriptors")]
impl crate::UnityEngine::Internal_SubsystemDescriptors {
    pub fn Internal_AddDescriptor(
        descriptor: quest_hook::libil2cpp::Gc<crate::UnityEngine::SubsystemDescriptor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_AddDescriptor", (descriptor))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Internal_SubsystemDescriptors")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Internal_SubsystemDescriptors {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
