#[cfg(feature = "OVR+OpenVR+OpenVRInterop")]
#[repr(C)]
#[derive(Debug)]
pub struct OpenVRInterop {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVR+OpenVR+OpenVRInterop")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::OpenVRInterop => "OVR.OpenVR"
    ."OpenVRInterop"
);
#[cfg(feature = "OVR+OpenVR+OpenVRInterop")]
impl std::ops::Deref for crate::OVR::OpenVR::OpenVRInterop {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVRInterop")]
impl std::ops::DerefMut for crate::OVR::OpenVR::OpenVRInterop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVRInterop")]
impl crate::OVR::OpenVR::OpenVRInterop {
    pub fn GetGenericInterface(
        pchInterfaceVersion: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGenericInterface", (pchInterfaceVersion, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInitToken() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInitToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringForHmdError(
        error: crate::OVR::OpenVR::EVRInitError,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStringForHmdError", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitInternal(
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
        eApplicationType: crate::OVR::OpenVR::EVRApplicationType,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitInternal", (peError, eApplicationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitInternal2(
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
        eApplicationType: crate::OVR::OpenVR::EVRApplicationType,
        pStartupInfo: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitInternal2", (peError, eApplicationType, pStartupInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHmdPresent() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHmdPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInterfaceVersionValid(
        pchInterfaceVersion: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInterfaceVersionValid", (pchInterfaceVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRuntimeInstalled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsRuntimeInstalled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ShutdownInternal() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShutdownInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVRInterop")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::OpenVRInterop {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
