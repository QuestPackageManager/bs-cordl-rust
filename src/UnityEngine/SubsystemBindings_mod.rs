#[cfg(feature = "UnityEngine+SubsystemBindings")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemBindings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SubsystemBindings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SubsystemBindings => "UnityEngine"
    ."SubsystemBindings"
);
#[cfg(feature = "UnityEngine+SubsystemBindings")]
impl std::ops::Deref for crate::UnityEngine::SubsystemBindings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemBindings")]
impl std::ops::DerefMut for crate::UnityEngine::SubsystemBindings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemBindings")]
impl crate::UnityEngine::SubsystemBindings {
    pub fn DestroySubsystem(
        nativePtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroySubsystem", (nativePtr))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SubsystemBindings")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SubsystemBindings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
