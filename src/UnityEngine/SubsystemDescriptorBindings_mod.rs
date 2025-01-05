#[cfg(feature = "UnityEngine+SubsystemDescriptorBindings")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemDescriptorBindings {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+SubsystemDescriptorBindings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SubsystemDescriptorBindings =>
    "UnityEngine"."SubsystemDescriptorBindings"
);
#[cfg(feature = "UnityEngine+SubsystemDescriptorBindings")]
impl std::ops::Deref for crate::UnityEngine::SubsystemDescriptorBindings {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
impl crate::UnityEngine::SubsystemDescriptorBindings {
    pub fn Create(
        descriptorPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (descriptorPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetId(
        descriptorPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetId", (descriptorPtr))?;
        Ok(__cordl_ret.into())
    }
}
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
