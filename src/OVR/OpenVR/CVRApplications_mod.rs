#[cfg(feature = "OVR+OpenVR+CVRApplications")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRApplications {
    __cordl_parent: crate::System::Object,
    pub FnTable: crate::OVR::OpenVR::IVRApplications,
}
#[cfg(feature = "OVR+OpenVR+CVRApplications")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRApplications => "OVR.OpenVR"
    ."CVRApplications"
);
#[cfg(feature = "OVR+OpenVR+CVRApplications")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRApplications {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRApplications")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRApplications {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRApplications")]
impl crate::OVR::OpenVR::CVRApplications {
    pub fn LaunchTemplateApplication(
        &mut self,
        pchTemplateAppKey: *mut crate::System::String,
        pchNewAppKey: *mut crate::System::String,
        pKeys: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::AppOverrideKeys_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke(
                "LaunchTemplateApplication",
                (pchTemplateAppKey, pchNewAppKey, pKeys),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetStartingApplication(
        &mut self,
        pchAppKeyBuffer: *mut crate::System::Text::StringBuilder,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("GetStartingApplication", (pchAppKeyBuffer, unAppKeyBufferLen))?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationKeyByProcessId(
        &mut self,
        unProcessId: u32,
        pchAppKeyBuffer: *mut crate::System::Text::StringBuilder,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke(
                "GetApplicationKeyByProcessId",
                (unProcessId, pchAppKeyBuffer, unAppKeyBufferLen),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IdentifyApplication(
        &mut self,
        unProcessId: u32,
        pchAppKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("IdentifyApplication", (unProcessId, pchAppKey))?;
        Ok(__cordl_ret)
    }
    pub fn LaunchInternalProcess(
        &mut self,
        pchBinaryPath: *mut crate::System::String,
        pchArguments: *mut crate::System::String,
        pchWorkingDirectory: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke(
                "LaunchInternalProcess",
                (pchBinaryPath, pchArguments, pchWorkingDirectory),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IsApplicationInstalled(
        &mut self,
        pchAppKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsApplicationInstalled", (pchAppKey))?;
        Ok(__cordl_ret)
    }
    pub fn CancelApplicationLaunch(
        &mut self,
        pchAppKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CancelApplicationLaunch", (pchAppKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationSupportedMimeTypes(
        &mut self,
        pchAppKey: *mut crate::System::String,
        pchMimeTypesBuffer: *mut crate::System::Text::StringBuilder,
        unMimeTypesBuffer: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetApplicationSupportedMimeTypes",
                (pchAppKey, pchMimeTypesBuffer, unMimeTypesBuffer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationsTransitionStateNameFromEnum(
        &mut self,
        state: crate::OVR::OpenVR::EVRApplicationTransitionState,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetApplicationsTransitionStateNameFromEnum", (state))?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationsErrorNameFromEnum(
        &mut self,
        error: crate::OVR::OpenVR::EVRApplicationError,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetApplicationsErrorNameFromEnum", (error))?;
        Ok(__cordl_ret)
    }
    pub fn GetTransitionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::OVR::OpenVR::EVRApplicationTransitionState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationTransitionState = __cordl_object
            .invoke("GetTransitionState", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultApplicationForMimeType(
        &mut self,
        pchMimeType: *mut crate::System::String,
        pchAppKeyBuffer: *mut crate::System::Text::StringBuilder,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetDefaultApplicationForMimeType",
                (pchMimeType, pchAppKeyBuffer, unAppKeyBufferLen),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LaunchApplicationFromMimeType(
        &mut self,
        pchMimeType: *mut crate::System::String,
        pchArgs: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("LaunchApplicationFromMimeType", (pchMimeType, pchArgs))?;
        Ok(__cordl_ret)
    }
    pub fn AddApplicationManifest(
        &mut self,
        pchApplicationManifestFullPath: *mut crate::System::String,
        bTemporary: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke(
                "AddApplicationManifest",
                (pchApplicationManifestFullPath, bTemporary),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationPropertyBool(
        &mut self,
        pchAppKey: *mut crate::System::String,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRApplicationError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetApplicationPropertyBool", (pchAppKey, eProperty, peError))?;
        Ok(__cordl_ret)
    }
    pub fn SetApplicationAutoLaunch(
        &mut self,
        pchAppKey: *mut crate::System::String,
        bAutoLaunch: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("SetApplicationAutoLaunch", (pchAppKey, bAutoLaunch))?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationPropertyUint64(
        &mut self,
        pchAppKey: *mut crate::System::String,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRApplicationError>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object
            .invoke("GetApplicationPropertyUint64", (pchAppKey, eProperty, peError))?;
        Ok(__cordl_ret)
    }
    pub fn IsQuitUserPromptRequested(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsQuitUserPromptRequested", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetDefaultApplicationForMimeType(
        &mut self,
        pchAppKey: *mut crate::System::String,
        pchMimeType: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("SetDefaultApplicationForMimeType", (pchAppKey, pchMimeType))?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationAutoLaunch(
        &mut self,
        pchAppKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetApplicationAutoLaunch", (pchAppKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pInterface))?;
        Ok(__cordl_ret)
    }
    pub fn LaunchDashboardOverlay(
        &mut self,
        pchAppKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("LaunchDashboardOverlay", (pchAppKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationProcessId(
        &mut self,
        pchAppKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetApplicationProcessId", (pchAppKey))?;
        Ok(__cordl_ret)
    }
    pub fn LaunchApplication(
        &mut self,
        pchAppKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("LaunchApplication", (pchAppKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentSceneProcessId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetCurrentSceneProcessId", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationLaunchArguments(
        &mut self,
        unHandle: u32,
        pchArgs: *mut crate::System::Text::StringBuilder,
        unArgs: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetApplicationLaunchArguments", (unHandle, pchArgs, unArgs))?;
        Ok(__cordl_ret)
    }
    pub fn PerformApplicationPrelaunchCheck(
        &mut self,
        pchAppKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("PerformApplicationPrelaunchCheck", (pchAppKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationCount(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetApplicationCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveApplicationManifest(
        &mut self,
        pchApplicationManifestFullPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("RemoveApplicationManifest", (pchApplicationManifestFullPath))?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationsThatSupportMimeType(
        &mut self,
        pchMimeType: *mut crate::System::String,
        pchAppKeysThatSupportBuffer: *mut crate::System::Text::StringBuilder,
        unAppKeysThatSupportBuffer: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetApplicationsThatSupportMimeType",
                (pchMimeType, pchAppKeysThatSupportBuffer, unAppKeysThatSupportBuffer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationPropertyString(
        &mut self,
        pchAppKey: *mut crate::System::String,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        pchPropertyValueBuffer: *mut crate::System::Text::StringBuilder,
        unPropertyValueBufferLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRApplicationError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetApplicationPropertyString",
                (
                    pchAppKey,
                    eProperty,
                    pchPropertyValueBuffer,
                    unPropertyValueBufferLen,
                    peError,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationKeyByIndex(
        &mut self,
        unApplicationIndex: u32,
        pchAppKeyBuffer: *mut crate::System::Text::StringBuilder,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke(
                "GetApplicationKeyByIndex",
                (unApplicationIndex, pchAppKeyBuffer, unAppKeyBufferLen),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRApplications")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRApplications {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
