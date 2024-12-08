#[cfg(feature = "UnityEngine+XR+XRInputSubsystem")]
#[repr(C)]
#[derive(Debug)]
pub struct XRInputSubsystem {
    __cordl_parent: crate::UnityEngine::IntegratedSubsystem_1<Blacklisted>,
    pub trackingOriginUpdated: *mut crate::System::Action_1<
        *mut crate::UnityEngine::XR::XRInputSubsystem,
    >,
    pub boundaryChanged: *mut crate::System::Action_1<
        *mut crate::UnityEngine::XR::XRInputSubsystem,
    >,
    pub m_DeviceIdsCache: *mut crate::System::Collections::Generic::List_1<u64>,
}
#[cfg(feature = "UnityEngine+XR+XRInputSubsystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::XRInputSubsystem =>
    "UnityEngine.XR"."XRInputSubsystem"
);
#[cfg(feature = "UnityEngine+XR+XRInputSubsystem")]
impl std::ops::Deref for crate::UnityEngine::XR::XRInputSubsystem {
    type Target = crate::UnityEngine::IntegratedSubsystem_1<Blacklisted>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+XRInputSubsystem")]
impl std::ops::DerefMut for crate::UnityEngine::XR::XRInputSubsystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+XRInputSubsystem")]
impl crate::UnityEngine::XR::XRInputSubsystem {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn TryRecenter(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryRecenter", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_boundaryChanged(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::XR::XRInputSubsystem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_boundaryChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_trackingOriginUpdated(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::XR::XRInputSubsystem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_trackingOriginUpdated", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_boundaryChanged(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::XR::XRInputSubsystem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_boundaryChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_trackingOriginUpdated(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::XR::XRInputSubsystem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_trackingOriginUpdated", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+XR+XRInputSubsystem")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::XRInputSubsystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
