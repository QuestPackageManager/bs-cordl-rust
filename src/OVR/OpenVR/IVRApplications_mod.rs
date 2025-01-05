#[cfg(feature = "OVR+OpenVR+IVRApplications")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IVRApplications {
    pub AddApplicationManifest: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__AddApplicationManifest,
    >,
    pub RemoveApplicationManifest: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__RemoveApplicationManifest,
    >,
    pub IsApplicationInstalled: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__IsApplicationInstalled,
    >,
    pub GetApplicationCount: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationCount,
    >,
    pub GetApplicationKeyByIndex: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByIndex,
    >,
    pub GetApplicationKeyByProcessId: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByProcessId,
    >,
    pub LaunchApplication: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__LaunchApplication,
    >,
    pub LaunchTemplateApplication: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__LaunchTemplateApplication,
    >,
    pub LaunchApplicationFromMimeType: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__LaunchApplicationFromMimeType,
    >,
    pub LaunchDashboardOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__LaunchDashboardOverlay,
    >,
    pub CancelApplicationLaunch: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__CancelApplicationLaunch,
    >,
    pub IdentifyApplication: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__IdentifyApplication,
    >,
    pub GetApplicationProcessId: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationProcessId,
    >,
    pub GetApplicationsErrorNameFromEnum: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationsErrorNameFromEnum,
    >,
    pub GetApplicationPropertyString: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyString,
    >,
    pub GetApplicationPropertyBool: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyBool,
    >,
    pub GetApplicationPropertyUint64: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyUint64,
    >,
    pub SetApplicationAutoLaunch: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__SetApplicationAutoLaunch,
    >,
    pub GetApplicationAutoLaunch: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationAutoLaunch,
    >,
    pub SetDefaultApplicationForMimeType: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__SetDefaultApplicationForMimeType,
    >,
    pub GetDefaultApplicationForMimeType: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetDefaultApplicationForMimeType,
    >,
    pub GetApplicationSupportedMimeTypes: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationSupportedMimeTypes,
    >,
    pub GetApplicationsThatSupportMimeType: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationsThatSupportMimeType,
    >,
    pub GetApplicationLaunchArguments: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationLaunchArguments,
    >,
    pub GetStartingApplication: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetStartingApplication,
    >,
    pub GetTransitionState: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetTransitionState,
    >,
    pub PerformApplicationPrelaunchCheck: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__PerformApplicationPrelaunchCheck,
    >,
    pub GetApplicationsTransitionStateNameFromEnum: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetApplicationsTransitionStateNameFromEnum,
    >,
    pub IsQuitUserPromptRequested: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__IsQuitUserPromptRequested,
    >,
    pub LaunchInternalProcess: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__LaunchInternalProcess,
    >,
    pub GetCurrentSceneProcessId: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRApplications__GetCurrentSceneProcessId,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRApplications => "OVR.OpenVR"
    ."IVRApplications"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVRApplications {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications")]
impl crate::OVR::OpenVR::IVRApplications {
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_AddApplicationManifest")]
    pub type _AddApplicationManifest = crate::OVR::OpenVR::IVRApplications__AddApplicationManifest;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_CancelApplicationLaunch")]
    pub type _CancelApplicationLaunch = crate::OVR::OpenVR::IVRApplications__CancelApplicationLaunch;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationAutoLaunch")]
    pub type _GetApplicationAutoLaunch = crate::OVR::OpenVR::IVRApplications__GetApplicationAutoLaunch;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationCount")]
    pub type _GetApplicationCount = crate::OVR::OpenVR::IVRApplications__GetApplicationCount;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByIndex")]
    pub type _GetApplicationKeyByIndex = crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByIndex;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByProcessId")]
    pub type _GetApplicationKeyByProcessId = crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByProcessId;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationLaunchArguments")]
    pub type _GetApplicationLaunchArguments = crate::OVR::OpenVR::IVRApplications__GetApplicationLaunchArguments;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationProcessId")]
    pub type _GetApplicationProcessId = crate::OVR::OpenVR::IVRApplications__GetApplicationProcessId;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyBool")]
    pub type _GetApplicationPropertyBool = crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyBool;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyString")]
    pub type _GetApplicationPropertyString = crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyString;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyUint64")]
    pub type _GetApplicationPropertyUint64 = crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyUint64;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationSupportedMimeTypes")]
    pub type _GetApplicationSupportedMimeTypes = crate::OVR::OpenVR::IVRApplications__GetApplicationSupportedMimeTypes;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsErrorNameFromEnum")]
    pub type _GetApplicationsErrorNameFromEnum = crate::OVR::OpenVR::IVRApplications__GetApplicationsErrorNameFromEnum;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsThatSupportMimeType")]
    pub type _GetApplicationsThatSupportMimeType = crate::OVR::OpenVR::IVRApplications__GetApplicationsThatSupportMimeType;
    #[cfg(
        feature = "OVR+OpenVR+IVRApplications+_GetApplicationsTransitionStateNameFromEnum"
    )]
    pub type _GetApplicationsTransitionStateNameFromEnum = crate::OVR::OpenVR::IVRApplications__GetApplicationsTransitionStateNameFromEnum;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetCurrentSceneProcessId")]
    pub type _GetCurrentSceneProcessId = crate::OVR::OpenVR::IVRApplications__GetCurrentSceneProcessId;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetDefaultApplicationForMimeType")]
    pub type _GetDefaultApplicationForMimeType = crate::OVR::OpenVR::IVRApplications__GetDefaultApplicationForMimeType;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetStartingApplication")]
    pub type _GetStartingApplication = crate::OVR::OpenVR::IVRApplications__GetStartingApplication;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_GetTransitionState")]
    pub type _GetTransitionState = crate::OVR::OpenVR::IVRApplications__GetTransitionState;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_IdentifyApplication")]
    pub type _IdentifyApplication = crate::OVR::OpenVR::IVRApplications__IdentifyApplication;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_IsApplicationInstalled")]
    pub type _IsApplicationInstalled = crate::OVR::OpenVR::IVRApplications__IsApplicationInstalled;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_IsQuitUserPromptRequested")]
    pub type _IsQuitUserPromptRequested = crate::OVR::OpenVR::IVRApplications__IsQuitUserPromptRequested;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplication")]
    pub type _LaunchApplication = crate::OVR::OpenVR::IVRApplications__LaunchApplication;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplicationFromMimeType")]
    pub type _LaunchApplicationFromMimeType = crate::OVR::OpenVR::IVRApplications__LaunchApplicationFromMimeType;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchDashboardOverlay")]
    pub type _LaunchDashboardOverlay = crate::OVR::OpenVR::IVRApplications__LaunchDashboardOverlay;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchInternalProcess")]
    pub type _LaunchInternalProcess = crate::OVR::OpenVR::IVRApplications__LaunchInternalProcess;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchTemplateApplication")]
    pub type _LaunchTemplateApplication = crate::OVR::OpenVR::IVRApplications__LaunchTemplateApplication;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_PerformApplicationPrelaunchCheck")]
    pub type _PerformApplicationPrelaunchCheck = crate::OVR::OpenVR::IVRApplications__PerformApplicationPrelaunchCheck;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_RemoveApplicationManifest")]
    pub type _RemoveApplicationManifest = crate::OVR::OpenVR::IVRApplications__RemoveApplicationManifest;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_SetApplicationAutoLaunch")]
    pub type _SetApplicationAutoLaunch = crate::OVR::OpenVR::IVRApplications__SetApplicationAutoLaunch;
    #[cfg(feature = "OVR+OpenVR+IVRApplications+_SetDefaultApplicationForMimeType")]
    pub type _SetDefaultApplicationForMimeType = crate::OVR::OpenVR::IVRApplications__SetDefaultApplicationForMimeType;
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_AddApplicationManifest")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__AddApplicationManifest {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_AddApplicationManifest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__AddApplicationManifest => "OVR.OpenVR"
    ."IVRApplications/_AddApplicationManifest"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_AddApplicationManifest")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__AddApplicationManifest {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_AddApplicationManifest")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRApplications__AddApplicationManifest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_AddApplicationManifest")]
impl crate::OVR::OpenVR::IVRApplications__AddApplicationManifest {
    pub fn BeginInvoke(
        &mut self,
        pchApplicationManifestFullPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        bTemporary: bool,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (pchApplicationManifestFullPath, bTemporary, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchApplicationManifestFullPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        bTemporary: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchApplicationManifestFullPath, bTemporary))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_AddApplicationManifest")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__AddApplicationManifest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_CancelApplicationLaunch")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__CancelApplicationLaunch {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_CancelApplicationLaunch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__CancelApplicationLaunch => "OVR.OpenVR"
    ."IVRApplications/_CancelApplicationLaunch"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_CancelApplicationLaunch")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__CancelApplicationLaunch {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_CancelApplicationLaunch")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__CancelApplicationLaunch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_CancelApplicationLaunch")]
impl crate::OVR::OpenVR::IVRApplications__CancelApplicationLaunch {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (pchAppKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_CancelApplicationLaunch")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__CancelApplicationLaunch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationAutoLaunch")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationAutoLaunch {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationAutoLaunch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationAutoLaunch => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationAutoLaunch"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationAutoLaunch")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__GetApplicationAutoLaunch {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationAutoLaunch")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationAutoLaunch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationAutoLaunch")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationAutoLaunch {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (pchAppKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationAutoLaunch")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationAutoLaunch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationCount")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationCount {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationCount")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationCount => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationCount"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationCount")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__GetApplicationCount {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationCount")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRApplications__GetApplicationCount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationCount")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationCount {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationCount")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationCount {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByIndex")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationKeyByIndex {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByIndex")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationKeyByIndex => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationKeyByIndex"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByIndex")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByIndex {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByIndex")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByIndex")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByIndex {
    pub fn BeginInvoke(
        &mut self,
        unApplicationIndex: u32,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    unApplicationIndex,
                    pchAppKeyBuffer,
                    unAppKeyBufferLen,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unApplicationIndex: u32,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (unApplicationIndex, pchAppKeyBuffer, unAppKeyBufferLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByIndex")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByIndex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByProcessId")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationKeyByProcessId {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByProcessId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationKeyByProcessId => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationKeyByProcessId"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByProcessId")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByProcessId {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByProcessId")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByProcessId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByProcessId")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByProcessId {
    pub fn BeginInvoke(
        &mut self,
        unProcessId: u32,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (unProcessId, pchAppKeyBuffer, unAppKeyBufferLen, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unProcessId: u32,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (unProcessId, pchAppKeyBuffer, unAppKeyBufferLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationKeyByProcessId")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationKeyByProcessId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationLaunchArguments")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationLaunchArguments {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationLaunchArguments")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationLaunchArguments => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationLaunchArguments"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationLaunchArguments")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__GetApplicationLaunchArguments {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationLaunchArguments")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationLaunchArguments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationLaunchArguments")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationLaunchArguments {
    pub fn BeginInvoke(
        &mut self,
        unHandle: u32,
        pchArgs: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unArgs: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (unHandle, pchArgs, unArgs, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unHandle: u32,
        pchArgs: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unArgs: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("Invoke", (unHandle, pchArgs, unArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationLaunchArguments")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationLaunchArguments {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationProcessId")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationProcessId {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationProcessId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationProcessId => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationProcessId"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationProcessId")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__GetApplicationProcessId {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationProcessId")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationProcessId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationProcessId")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationProcessId {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("Invoke", (pchAppKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationProcessId")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationProcessId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyBool")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationPropertyBool {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyBool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationPropertyBool => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationPropertyBool"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyBool")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyBool {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyBool")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyBool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyBool")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyBool {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        peError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRApplicationError,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, eProperty, peError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRApplicationError,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRApplicationError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pchAppKey, eProperty, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyBool")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyBool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyString")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationPropertyString {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyString")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationPropertyString => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationPropertyString"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyString")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyString {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyString")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyString")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyString {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        pchPropertyValueBuffer: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unPropertyValueBufferLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRApplicationError,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    pchAppKey,
                    eProperty,
                    pchPropertyValueBuffer,
                    unPropertyValueBufferLen,
                    peError,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRApplicationError,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        pchPropertyValueBuffer: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unPropertyValueBufferLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRApplicationError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "Invoke",
                (
                    pchAppKey,
                    eProperty,
                    pchPropertyValueBuffer,
                    unPropertyValueBufferLen,
                    peError,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyString")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyString {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyUint64")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationPropertyUint64 {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyUint64")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationPropertyUint64 => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationPropertyUint64"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyUint64")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyUint64 {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyUint64")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyUint64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyUint64")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyUint64 {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        peError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRApplicationError,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, eProperty, peError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRApplicationError,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRApplicationError>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object
            .invoke("Invoke", (pchAppKey, eProperty, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationPropertyUint64")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationPropertyUint64 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationSupportedMimeTypes")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationSupportedMimeTypes {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationSupportedMimeTypes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationSupportedMimeTypes => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationSupportedMimeTypes"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationSupportedMimeTypes")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__GetApplicationSupportedMimeTypes {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationSupportedMimeTypes")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationSupportedMimeTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationSupportedMimeTypes")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationSupportedMimeTypes {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchMimeTypesBuffer: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unMimeTypesBuffer: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (pchAppKey, pchMimeTypesBuffer, unMimeTypesBuffer, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchMimeTypesBuffer: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unMimeTypesBuffer: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pchAppKey, pchMimeTypesBuffer, unMimeTypesBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationSupportedMimeTypes")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationSupportedMimeTypes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsErrorNameFromEnum")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationsErrorNameFromEnum {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsErrorNameFromEnum")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationsErrorNameFromEnum => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationsErrorNameFromEnum"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsErrorNameFromEnum")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__GetApplicationsErrorNameFromEnum {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsErrorNameFromEnum")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationsErrorNameFromEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsErrorNameFromEnum")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationsErrorNameFromEnum {
    pub fn BeginInvoke(
        &mut self,
        error: crate::OVR::OpenVR::EVRApplicationError,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (error, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        error: crate::OVR::OpenVR::EVRApplicationError,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsErrorNameFromEnum")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationsErrorNameFromEnum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsThatSupportMimeType")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationsThatSupportMimeType {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsThatSupportMimeType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationsThatSupportMimeType => "OVR.OpenVR"
    ."IVRApplications/_GetApplicationsThatSupportMimeType"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsThatSupportMimeType")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__GetApplicationsThatSupportMimeType {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsThatSupportMimeType")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationsThatSupportMimeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsThatSupportMimeType")]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationsThatSupportMimeType {
    pub fn BeginInvoke(
        &mut self,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchAppKeysThatSupportBuffer: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unAppKeysThatSupportBuffer: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    pchMimeType,
                    pchAppKeysThatSupportBuffer,
                    unAppKeysThatSupportBuffer,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchAppKeysThatSupportBuffer: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unAppKeysThatSupportBuffer: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "Invoke",
                (pchMimeType, pchAppKeysThatSupportBuffer, unAppKeysThatSupportBuffer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetApplicationsThatSupportMimeType")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationsThatSupportMimeType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRApplications+_GetApplicationsTransitionStateNameFromEnum"
)]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetApplicationsTransitionStateNameFromEnum {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "OVR+OpenVR+IVRApplications+_GetApplicationsTransitionStateNameFromEnum"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetApplicationsTransitionStateNameFromEnum =>
    "OVR.OpenVR"."IVRApplications/_GetApplicationsTransitionStateNameFromEnum"
);
#[cfg(
    feature = "OVR+OpenVR+IVRApplications+_GetApplicationsTransitionStateNameFromEnum"
)]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__GetApplicationsTransitionStateNameFromEnum {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRApplications+_GetApplicationsTransitionStateNameFromEnum"
)]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetApplicationsTransitionStateNameFromEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRApplications+_GetApplicationsTransitionStateNameFromEnum"
)]
impl crate::OVR::OpenVR::IVRApplications__GetApplicationsTransitionStateNameFromEnum {
    pub fn BeginInvoke(
        &mut self,
        state: crate::OVR::OpenVR::EVRApplicationTransitionState,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (state, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        state: crate::OVR::OpenVR::EVRApplicationTransitionState,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRApplications+_GetApplicationsTransitionStateNameFromEnum"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetApplicationsTransitionStateNameFromEnum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetCurrentSceneProcessId")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetCurrentSceneProcessId {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetCurrentSceneProcessId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetCurrentSceneProcessId => "OVR.OpenVR"
    ."IVRApplications/_GetCurrentSceneProcessId"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetCurrentSceneProcessId")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__GetCurrentSceneProcessId {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetCurrentSceneProcessId")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetCurrentSceneProcessId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetCurrentSceneProcessId")]
impl crate::OVR::OpenVR::IVRApplications__GetCurrentSceneProcessId {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetCurrentSceneProcessId")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetCurrentSceneProcessId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetDefaultApplicationForMimeType")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetDefaultApplicationForMimeType {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetDefaultApplicationForMimeType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetDefaultApplicationForMimeType => "OVR.OpenVR"
    ."IVRApplications/_GetDefaultApplicationForMimeType"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetDefaultApplicationForMimeType")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__GetDefaultApplicationForMimeType {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetDefaultApplicationForMimeType")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__GetDefaultApplicationForMimeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetDefaultApplicationForMimeType")]
impl crate::OVR::OpenVR::IVRApplications__GetDefaultApplicationForMimeType {
    pub fn BeginInvoke(
        &mut self,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (pchMimeType, pchAppKeyBuffer, unAppKeyBufferLen, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pchMimeType, pchAppKeyBuffer, unAppKeyBufferLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetDefaultApplicationForMimeType")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetDefaultApplicationForMimeType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetStartingApplication")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetStartingApplication {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetStartingApplication")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__GetStartingApplication => "OVR.OpenVR"
    ."IVRApplications/_GetStartingApplication"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetStartingApplication")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__GetStartingApplication {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetStartingApplication")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRApplications__GetStartingApplication {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetStartingApplication")]
impl crate::OVR::OpenVR::IVRApplications__GetStartingApplication {
    pub fn BeginInvoke(
        &mut self,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (pchAppKeyBuffer, unAppKeyBufferLen, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchAppKeyBuffer, unAppKeyBufferLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetStartingApplication")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetStartingApplication {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetTransitionState")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__GetTransitionState {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetTransitionState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRApplications__GetTransitionState
    => "OVR.OpenVR"."IVRApplications/_GetTransitionState"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetTransitionState")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__GetTransitionState {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetTransitionState")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRApplications__GetTransitionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetTransitionState")]
impl crate::OVR::OpenVR::IVRApplications__GetTransitionState {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        crate::OVR::OpenVR::EVRApplicationTransitionState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationTransitionState = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::OVR::OpenVR::EVRApplicationTransitionState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationTransitionState = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_GetTransitionState")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__GetTransitionState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IdentifyApplication")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__IdentifyApplication {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IdentifyApplication")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__IdentifyApplication => "OVR.OpenVR"
    ."IVRApplications/_IdentifyApplication"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IdentifyApplication")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__IdentifyApplication {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IdentifyApplication")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRApplications__IdentifyApplication {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IdentifyApplication")]
impl crate::OVR::OpenVR::IVRApplications__IdentifyApplication {
    pub fn BeginInvoke(
        &mut self,
        unProcessId: u32,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (unProcessId, pchAppKey, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unProcessId: u32,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (unProcessId, pchAppKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IdentifyApplication")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__IdentifyApplication {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsApplicationInstalled")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__IsApplicationInstalled {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsApplicationInstalled")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__IsApplicationInstalled => "OVR.OpenVR"
    ."IVRApplications/_IsApplicationInstalled"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsApplicationInstalled")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__IsApplicationInstalled {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsApplicationInstalled")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRApplications__IsApplicationInstalled {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsApplicationInstalled")]
impl crate::OVR::OpenVR::IVRApplications__IsApplicationInstalled {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (pchAppKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsApplicationInstalled")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__IsApplicationInstalled {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsQuitUserPromptRequested")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__IsQuitUserPromptRequested {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsQuitUserPromptRequested")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__IsQuitUserPromptRequested => "OVR.OpenVR"
    ."IVRApplications/_IsQuitUserPromptRequested"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsQuitUserPromptRequested")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__IsQuitUserPromptRequested {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsQuitUserPromptRequested")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__IsQuitUserPromptRequested {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsQuitUserPromptRequested")]
impl crate::OVR::OpenVR::IVRApplications__IsQuitUserPromptRequested {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_IsQuitUserPromptRequested")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__IsQuitUserPromptRequested {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplication")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__LaunchApplication {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplication")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRApplications__LaunchApplication
    => "OVR.OpenVR"."IVRApplications/_LaunchApplication"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplication")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__LaunchApplication {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplication")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRApplications__LaunchApplication {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplication")]
impl crate::OVR::OpenVR::IVRApplications__LaunchApplication {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchAppKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplication")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__LaunchApplication {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplicationFromMimeType")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__LaunchApplicationFromMimeType {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplicationFromMimeType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__LaunchApplicationFromMimeType => "OVR.OpenVR"
    ."IVRApplications/_LaunchApplicationFromMimeType"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplicationFromMimeType")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__LaunchApplicationFromMimeType {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplicationFromMimeType")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__LaunchApplicationFromMimeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplicationFromMimeType")]
impl crate::OVR::OpenVR::IVRApplications__LaunchApplicationFromMimeType {
    pub fn BeginInvoke(
        &mut self,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchArgs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchMimeType, pchArgs, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchArgs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchMimeType, pchArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchApplicationFromMimeType")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__LaunchApplicationFromMimeType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchDashboardOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__LaunchDashboardOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchDashboardOverlay")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__LaunchDashboardOverlay => "OVR.OpenVR"
    ."IVRApplications/_LaunchDashboardOverlay"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchDashboardOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__LaunchDashboardOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchDashboardOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRApplications__LaunchDashboardOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchDashboardOverlay")]
impl crate::OVR::OpenVR::IVRApplications__LaunchDashboardOverlay {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchAppKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchDashboardOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__LaunchDashboardOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchInternalProcess")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__LaunchInternalProcess {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchInternalProcess")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__LaunchInternalProcess => "OVR.OpenVR"
    ."IVRApplications/_LaunchInternalProcess"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchInternalProcess")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__LaunchInternalProcess {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchInternalProcess")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRApplications__LaunchInternalProcess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchInternalProcess")]
impl crate::OVR::OpenVR::IVRApplications__LaunchInternalProcess {
    pub fn BeginInvoke(
        &mut self,
        pchBinaryPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchArguments: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchWorkingDirectory: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (pchBinaryPath, pchArguments, pchWorkingDirectory, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchBinaryPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchArguments: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchWorkingDirectory: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchBinaryPath, pchArguments, pchWorkingDirectory))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchInternalProcess")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__LaunchInternalProcess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchTemplateApplication")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__LaunchTemplateApplication {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchTemplateApplication")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__LaunchTemplateApplication => "OVR.OpenVR"
    ."IVRApplications/_LaunchTemplateApplication"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchTemplateApplication")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__LaunchTemplateApplication {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchTemplateApplication")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__LaunchTemplateApplication {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchTemplateApplication")]
impl crate::OVR::OpenVR::IVRApplications__LaunchTemplateApplication {
    pub fn BeginInvoke(
        &mut self,
        pchTemplateAppKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchNewAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pKeys: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::AppOverrideKeys_t>,
            >,
        >,
        unKeys: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (pchTemplateAppKey, pchNewAppKey, pKeys, unKeys, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchTemplateAppKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchNewAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pKeys: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::AppOverrideKeys_t>,
            >,
        >,
        unKeys: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchTemplateAppKey, pchNewAppKey, pKeys, unKeys))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_LaunchTemplateApplication")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__LaunchTemplateApplication {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_PerformApplicationPrelaunchCheck")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__PerformApplicationPrelaunchCheck {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_PerformApplicationPrelaunchCheck")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__PerformApplicationPrelaunchCheck => "OVR.OpenVR"
    ."IVRApplications/_PerformApplicationPrelaunchCheck"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_PerformApplicationPrelaunchCheck")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__PerformApplicationPrelaunchCheck {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_PerformApplicationPrelaunchCheck")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__PerformApplicationPrelaunchCheck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_PerformApplicationPrelaunchCheck")]
impl crate::OVR::OpenVR::IVRApplications__PerformApplicationPrelaunchCheck {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchAppKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_PerformApplicationPrelaunchCheck")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__PerformApplicationPrelaunchCheck {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_RemoveApplicationManifest")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__RemoveApplicationManifest {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_RemoveApplicationManifest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__RemoveApplicationManifest => "OVR.OpenVR"
    ."IVRApplications/_RemoveApplicationManifest"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_RemoveApplicationManifest")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__RemoveApplicationManifest {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_RemoveApplicationManifest")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__RemoveApplicationManifest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_RemoveApplicationManifest")]
impl crate::OVR::OpenVR::IVRApplications__RemoveApplicationManifest {
    pub fn BeginInvoke(
        &mut self,
        pchApplicationManifestFullPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchApplicationManifestFullPath, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchApplicationManifestFullPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchApplicationManifestFullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_RemoveApplicationManifest")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__RemoveApplicationManifest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetApplicationAutoLaunch")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__SetApplicationAutoLaunch {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetApplicationAutoLaunch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__SetApplicationAutoLaunch => "OVR.OpenVR"
    ."IVRApplications/_SetApplicationAutoLaunch"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetApplicationAutoLaunch")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRApplications__SetApplicationAutoLaunch {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetApplicationAutoLaunch")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__SetApplicationAutoLaunch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetApplicationAutoLaunch")]
impl crate::OVR::OpenVR::IVRApplications__SetApplicationAutoLaunch {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bAutoLaunch: bool,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, bAutoLaunch, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bAutoLaunch: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchAppKey, bAutoLaunch))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetApplicationAutoLaunch")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__SetApplicationAutoLaunch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetDefaultApplicationForMimeType")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRApplications__SetDefaultApplicationForMimeType {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetDefaultApplicationForMimeType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRApplications__SetDefaultApplicationForMimeType => "OVR.OpenVR"
    ."IVRApplications/_SetDefaultApplicationForMimeType"
);
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetDefaultApplicationForMimeType")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRApplications__SetDefaultApplicationForMimeType {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetDefaultApplicationForMimeType")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRApplications__SetDefaultApplicationForMimeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetDefaultApplicationForMimeType")]
impl crate::OVR::OpenVR::IVRApplications__SetDefaultApplicationForMimeType {
    pub fn BeginInvoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchAppKey, pchMimeType, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = __cordl_object
            .invoke("Invoke", (pchAppKey, pchMimeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRApplications+_SetDefaultApplicationForMimeType")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRApplications__SetDefaultApplicationForMimeType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
