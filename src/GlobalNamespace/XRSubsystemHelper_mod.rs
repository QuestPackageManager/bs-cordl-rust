#[cfg(feature = "XRSubsystemHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct XRSubsystemHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "XRSubsystemHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::XRSubsystemHelper => ""
    ."XRSubsystemHelper"
);
#[cfg(feature = "XRSubsystemHelper")]
impl std::ops::Deref for crate::GlobalNamespace::XRSubsystemHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "XRSubsystemHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::XRSubsystemHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "XRSubsystemHelper")]
impl crate::GlobalNamespace::XRSubsystemHelper {
    pub fn GetCurrentDisplaySubsystem() -> quest_hook::libil2cpp::Result<Blacklisted> {
        let __cordl_ret: Blacklisted = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentDisplaySubsystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentDisplaySubsystemDescriptor() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRDisplaySubsystemDescriptor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::XRDisplaySubsystemDescriptor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentDisplaySubsystemDescriptor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentInputSubsystem() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRInputSubsystem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::XRInputSubsystem,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentInputSubsystem", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "XRSubsystemHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::XRSubsystemHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
