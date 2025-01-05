#[cfg(feature = "OVR+OpenVR+IVRSystem")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IVRSystem {
    pub GetRecommendedRenderTargetSize: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetRecommendedRenderTargetSize,
    >,
    pub GetProjectionMatrix: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetProjectionMatrix,
    >,
    pub GetProjectionRaw: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetProjectionRaw,
    >,
    pub ComputeDistortion: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__ComputeDistortion,
    >,
    pub GetEyeToHeadTransform: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetEyeToHeadTransform,
    >,
    pub GetTimeSinceLastVsync: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetTimeSinceLastVsync,
    >,
    pub GetD3D9AdapterIndex: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetD3D9AdapterIndex,
    >,
    pub GetDXGIOutputInfo: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetDXGIOutputInfo,
    >,
    pub GetOutputDevice: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetOutputDevice,
    >,
    pub IsDisplayOnDesktop: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__IsDisplayOnDesktop,
    >,
    pub SetDisplayVisibility: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__SetDisplayVisibility,
    >,
    pub GetDeviceToAbsoluteTrackingPose: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetDeviceToAbsoluteTrackingPose,
    >,
    pub ResetSeatedZeroPose: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__ResetSeatedZeroPose,
    >,
    pub GetSeatedZeroPoseToStandingAbsoluteTrackingPose: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetSeatedZeroPoseToStandingAbsoluteTrackingPose,
    >,
    pub GetRawZeroPoseToStandingAbsoluteTrackingPose: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetRawZeroPoseToStandingAbsoluteTrackingPose,
    >,
    pub GetSortedTrackedDeviceIndicesOfClass: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetSortedTrackedDeviceIndicesOfClass,
    >,
    pub GetTrackedDeviceActivityLevel: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceActivityLevel,
    >,
    pub ApplyTransform: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__ApplyTransform,
    >,
    pub GetTrackedDeviceIndexForControllerRole: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceIndexForControllerRole,
    >,
    pub GetControllerRoleForTrackedDeviceIndex: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetControllerRoleForTrackedDeviceIndex,
    >,
    pub GetTrackedDeviceClass: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceClass,
    >,
    pub IsTrackedDeviceConnected: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__IsTrackedDeviceConnected,
    >,
    pub GetBoolTrackedDeviceProperty: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetBoolTrackedDeviceProperty,
    >,
    pub GetFloatTrackedDeviceProperty: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetFloatTrackedDeviceProperty,
    >,
    pub GetInt32TrackedDeviceProperty: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetInt32TrackedDeviceProperty,
    >,
    pub GetUint64TrackedDeviceProperty: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetUint64TrackedDeviceProperty,
    >,
    pub GetMatrix34TrackedDeviceProperty: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetMatrix34TrackedDeviceProperty,
    >,
    pub GetArrayTrackedDeviceProperty: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetArrayTrackedDeviceProperty,
    >,
    pub GetStringTrackedDeviceProperty: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetStringTrackedDeviceProperty,
    >,
    pub GetPropErrorNameFromEnum: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetPropErrorNameFromEnum,
    >,
    pub PollNextEvent: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__PollNextEvent,
    >,
    pub PollNextEventWithPose: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__PollNextEventWithPose,
    >,
    pub GetEventTypeNameFromEnum: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetEventTypeNameFromEnum,
    >,
    pub GetHiddenAreaMesh: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetHiddenAreaMesh,
    >,
    pub GetControllerState: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetControllerState,
    >,
    pub GetControllerStateWithPose: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetControllerStateWithPose,
    >,
    pub TriggerHapticPulse: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__TriggerHapticPulse,
    >,
    pub GetButtonIdNameFromEnum: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetButtonIdNameFromEnum,
    >,
    pub GetControllerAxisTypeNameFromEnum: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__GetControllerAxisTypeNameFromEnum,
    >,
    pub IsInputAvailable: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__IsInputAvailable,
    >,
    pub IsSteamVRDrawingControllers: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__IsSteamVRDrawingControllers,
    >,
    pub ShouldApplicationPause: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__ShouldApplicationPause,
    >,
    pub ShouldApplicationReduceRenderingWork: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__ShouldApplicationReduceRenderingWork,
    >,
    pub DriverDebugRequest: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__DriverDebugRequest,
    >,
    pub PerformFirmwareUpdate: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__PerformFirmwareUpdate,
    >,
    pub AcknowledgeQuit_Exiting: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_Exiting,
    >,
    pub AcknowledgeQuit_UserPrompt: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_UserPrompt,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem => "OVR.OpenVR"
    ."IVRSystem"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVRSystem {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem")]
impl crate::OVR::OpenVR::IVRSystem {
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_Exiting")]
    pub type _AcknowledgeQuit_Exiting = crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_Exiting;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_UserPrompt")]
    pub type _AcknowledgeQuit_UserPrompt = crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_UserPrompt;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_ApplyTransform")]
    pub type _ApplyTransform = crate::OVR::OpenVR::IVRSystem__ApplyTransform;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_ComputeDistortion")]
    pub type _ComputeDistortion = crate::OVR::OpenVR::IVRSystem__ComputeDistortion;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_DriverDebugRequest")]
    pub type _DriverDebugRequest = crate::OVR::OpenVR::IVRSystem__DriverDebugRequest;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetArrayTrackedDeviceProperty")]
    pub type _GetArrayTrackedDeviceProperty = crate::OVR::OpenVR::IVRSystem__GetArrayTrackedDeviceProperty;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetBoolTrackedDeviceProperty")]
    pub type _GetBoolTrackedDeviceProperty = crate::OVR::OpenVR::IVRSystem__GetBoolTrackedDeviceProperty;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetButtonIdNameFromEnum")]
    pub type _GetButtonIdNameFromEnum = crate::OVR::OpenVR::IVRSystem__GetButtonIdNameFromEnum;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerAxisTypeNameFromEnum")]
    pub type _GetControllerAxisTypeNameFromEnum = crate::OVR::OpenVR::IVRSystem__GetControllerAxisTypeNameFromEnum;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerRoleForTrackedDeviceIndex")]
    pub type _GetControllerRoleForTrackedDeviceIndex = crate::OVR::OpenVR::IVRSystem__GetControllerRoleForTrackedDeviceIndex;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerState")]
    pub type _GetControllerState = crate::OVR::OpenVR::IVRSystem__GetControllerState;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerStateWithPose")]
    pub type _GetControllerStateWithPose = crate::OVR::OpenVR::IVRSystem__GetControllerStateWithPose;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetD3D9AdapterIndex")]
    pub type _GetD3D9AdapterIndex = crate::OVR::OpenVR::IVRSystem__GetD3D9AdapterIndex;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDXGIOutputInfo")]
    pub type _GetDXGIOutputInfo = crate::OVR::OpenVR::IVRSystem__GetDXGIOutputInfo;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDeviceToAbsoluteTrackingPose")]
    pub type _GetDeviceToAbsoluteTrackingPose = crate::OVR::OpenVR::IVRSystem__GetDeviceToAbsoluteTrackingPose;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEventTypeNameFromEnum")]
    pub type _GetEventTypeNameFromEnum = crate::OVR::OpenVR::IVRSystem__GetEventTypeNameFromEnum;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEyeToHeadTransform")]
    pub type _GetEyeToHeadTransform = crate::OVR::OpenVR::IVRSystem__GetEyeToHeadTransform;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetFloatTrackedDeviceProperty")]
    pub type _GetFloatTrackedDeviceProperty = crate::OVR::OpenVR::IVRSystem__GetFloatTrackedDeviceProperty;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetHiddenAreaMesh")]
    pub type _GetHiddenAreaMesh = crate::OVR::OpenVR::IVRSystem__GetHiddenAreaMesh;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetInt32TrackedDeviceProperty")]
    pub type _GetInt32TrackedDeviceProperty = crate::OVR::OpenVR::IVRSystem__GetInt32TrackedDeviceProperty;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetMatrix34TrackedDeviceProperty")]
    pub type _GetMatrix34TrackedDeviceProperty = crate::OVR::OpenVR::IVRSystem__GetMatrix34TrackedDeviceProperty;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetOutputDevice")]
    pub type _GetOutputDevice = crate::OVR::OpenVR::IVRSystem__GetOutputDevice;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionMatrix")]
    pub type _GetProjectionMatrix = crate::OVR::OpenVR::IVRSystem__GetProjectionMatrix;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionRaw")]
    pub type _GetProjectionRaw = crate::OVR::OpenVR::IVRSystem__GetProjectionRaw;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetPropErrorNameFromEnum")]
    pub type _GetPropErrorNameFromEnum = crate::OVR::OpenVR::IVRSystem__GetPropErrorNameFromEnum;
    #[cfg(
        feature = "OVR+OpenVR+IVRSystem+_GetRawZeroPoseToStandingAbsoluteTrackingPose"
    )]
    pub type _GetRawZeroPoseToStandingAbsoluteTrackingPose = crate::OVR::OpenVR::IVRSystem__GetRawZeroPoseToStandingAbsoluteTrackingPose;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRecommendedRenderTargetSize")]
    pub type _GetRecommendedRenderTargetSize = crate::OVR::OpenVR::IVRSystem__GetRecommendedRenderTargetSize;
    #[cfg(
        feature = "OVR+OpenVR+IVRSystem+_GetSeatedZeroPoseToStandingAbsoluteTrackingPose"
    )]
    pub type _GetSeatedZeroPoseToStandingAbsoluteTrackingPose = crate::OVR::OpenVR::IVRSystem__GetSeatedZeroPoseToStandingAbsoluteTrackingPose;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSortedTrackedDeviceIndicesOfClass")]
    pub type _GetSortedTrackedDeviceIndicesOfClass = crate::OVR::OpenVR::IVRSystem__GetSortedTrackedDeviceIndicesOfClass;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetStringTrackedDeviceProperty")]
    pub type _GetStringTrackedDeviceProperty = crate::OVR::OpenVR::IVRSystem__GetStringTrackedDeviceProperty;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTimeSinceLastVsync")]
    pub type _GetTimeSinceLastVsync = crate::OVR::OpenVR::IVRSystem__GetTimeSinceLastVsync;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceActivityLevel")]
    pub type _GetTrackedDeviceActivityLevel = crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceActivityLevel;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceClass")]
    pub type _GetTrackedDeviceClass = crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceClass;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceIndexForControllerRole")]
    pub type _GetTrackedDeviceIndexForControllerRole = crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceIndexForControllerRole;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_GetUint64TrackedDeviceProperty")]
    pub type _GetUint64TrackedDeviceProperty = crate::OVR::OpenVR::IVRSystem__GetUint64TrackedDeviceProperty;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_IsDisplayOnDesktop")]
    pub type _IsDisplayOnDesktop = crate::OVR::OpenVR::IVRSystem__IsDisplayOnDesktop;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_IsInputAvailable")]
    pub type _IsInputAvailable = crate::OVR::OpenVR::IVRSystem__IsInputAvailable;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_IsSteamVRDrawingControllers")]
    pub type _IsSteamVRDrawingControllers = crate::OVR::OpenVR::IVRSystem__IsSteamVRDrawingControllers;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_IsTrackedDeviceConnected")]
    pub type _IsTrackedDeviceConnected = crate::OVR::OpenVR::IVRSystem__IsTrackedDeviceConnected;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_PerformFirmwareUpdate")]
    pub type _PerformFirmwareUpdate = crate::OVR::OpenVR::IVRSystem__PerformFirmwareUpdate;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEvent")]
    pub type _PollNextEvent = crate::OVR::OpenVR::IVRSystem__PollNextEvent;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEventWithPose")]
    pub type _PollNextEventWithPose = crate::OVR::OpenVR::IVRSystem__PollNextEventWithPose;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_ResetSeatedZeroPose")]
    pub type _ResetSeatedZeroPose = crate::OVR::OpenVR::IVRSystem__ResetSeatedZeroPose;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_SetDisplayVisibility")]
    pub type _SetDisplayVisibility = crate::OVR::OpenVR::IVRSystem__SetDisplayVisibility;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationPause")]
    pub type _ShouldApplicationPause = crate::OVR::OpenVR::IVRSystem__ShouldApplicationPause;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationReduceRenderingWork")]
    pub type _ShouldApplicationReduceRenderingWork = crate::OVR::OpenVR::IVRSystem__ShouldApplicationReduceRenderingWork;
    #[cfg(feature = "OVR+OpenVR+IVRSystem+_TriggerHapticPulse")]
    pub type _TriggerHapticPulse = crate::OVR::OpenVR::IVRSystem__TriggerHapticPulse;
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_Exiting")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__AcknowledgeQuit_Exiting {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_Exiting")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__AcknowledgeQuit_Exiting
    => "OVR.OpenVR"."IVRSystem/_AcknowledgeQuit_Exiting"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_Exiting")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_Exiting {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_Exiting")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_Exiting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_Exiting")]
impl crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_Exiting {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_Exiting")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_Exiting {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_UserPrompt")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__AcknowledgeQuit_UserPrompt {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_UserPrompt")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__AcknowledgeQuit_UserPrompt => "OVR.OpenVR"
    ."IVRSystem/_AcknowledgeQuit_UserPrompt"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_UserPrompt")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_UserPrompt {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_UserPrompt")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_UserPrompt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_UserPrompt")]
impl crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_UserPrompt {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_AcknowledgeQuit_UserPrompt")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__AcknowledgeQuit_UserPrompt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ApplyTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__ApplyTransform {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ApplyTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__ApplyTransform =>
    "OVR.OpenVR"."IVRSystem/_ApplyTransform"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ApplyTransform")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__ApplyTransform {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ApplyTransform")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__ApplyTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ApplyTransform")]
impl crate::OVR::OpenVR::IVRSystem__ApplyTransform {
    pub fn BeginInvoke(
        &mut self,
        pOutputPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pTransform: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>,
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
                (pOutputPose, pTrackedDevicePose, pTransform, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pOutputPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pTransform: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pOutputPose, pTrackedDevicePose, pTransform, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pOutputPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pTransform: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pOutputPose, pTrackedDevicePose, pTransform))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ApplyTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__ApplyTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ComputeDistortion")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__ComputeDistortion {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ComputeDistortion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__ComputeDistortion =>
    "OVR.OpenVR"."IVRSystem/_ComputeDistortion"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ComputeDistortion")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__ComputeDistortion {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ComputeDistortion")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__ComputeDistortion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ComputeDistortion")]
impl crate::OVR::OpenVR::IVRSystem__ComputeDistortion {
    pub fn BeginInvoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        fU: f32,
        fV: f32,
        pDistortionCoordinates: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::DistortionCoordinates_t,
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
                (eEye, fU, fV, pDistortionCoordinates, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pDistortionCoordinates: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::DistortionCoordinates_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pDistortionCoordinates, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        fU: f32,
        fV: f32,
        pDistortionCoordinates: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::DistortionCoordinates_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (eEye, fU, fV, pDistortionCoordinates))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ComputeDistortion")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__ComputeDistortion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_DriverDebugRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__DriverDebugRequest {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_DriverDebugRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__DriverDebugRequest =>
    "OVR.OpenVR"."IVRSystem/_DriverDebugRequest"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_DriverDebugRequest")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__DriverDebugRequest {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_DriverDebugRequest")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__DriverDebugRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_DriverDebugRequest")]
impl crate::OVR::OpenVR::IVRSystem__DriverDebugRequest {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        pchRequest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchResponseBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unResponseBufferSize: u32,
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
                    unDeviceIndex,
                    pchRequest,
                    pchResponseBuffer,
                    unResponseBufferSize,
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
        unDeviceIndex: u32,
        pchRequest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchResponseBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unResponseBufferSize: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "Invoke",
                (unDeviceIndex, pchRequest, pchResponseBuffer, unResponseBufferSize),
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_DriverDebugRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__DriverDebugRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetArrayTrackedDeviceProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetArrayTrackedDeviceProperty {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetArrayTrackedDeviceProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetArrayTrackedDeviceProperty => "OVR.OpenVR"
    ."IVRSystem/_GetArrayTrackedDeviceProperty"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetArrayTrackedDeviceProperty")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetArrayTrackedDeviceProperty {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetArrayTrackedDeviceProperty")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetArrayTrackedDeviceProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetArrayTrackedDeviceProperty")]
impl crate::OVR::OpenVR::IVRSystem__GetArrayTrackedDeviceProperty {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        propType: u32,
        pBuffer: crate::System::IntPtr,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
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
                    unDeviceIndex,
                    prop,
                    propType,
                    pBuffer,
                    unBufferSize,
                    pError,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        propType: u32,
        pBuffer: crate::System::IntPtr,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "Invoke",
                (unDeviceIndex, prop, propType, pBuffer, unBufferSize, pError),
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetArrayTrackedDeviceProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetArrayTrackedDeviceProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetBoolTrackedDeviceProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetBoolTrackedDeviceProperty {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetBoolTrackedDeviceProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetBoolTrackedDeviceProperty => "OVR.OpenVR"
    ."IVRSystem/_GetBoolTrackedDeviceProperty"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetBoolTrackedDeviceProperty")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetBoolTrackedDeviceProperty {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetBoolTrackedDeviceProperty")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetBoolTrackedDeviceProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetBoolTrackedDeviceProperty")]
impl crate::OVR::OpenVR::IVRSystem__GetBoolTrackedDeviceProperty {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
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
            .invoke("BeginInvoke", (unDeviceIndex, prop, pError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (unDeviceIndex, prop, pError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetBoolTrackedDeviceProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetBoolTrackedDeviceProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetButtonIdNameFromEnum")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetButtonIdNameFromEnum {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetButtonIdNameFromEnum")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetButtonIdNameFromEnum
    => "OVR.OpenVR"."IVRSystem/_GetButtonIdNameFromEnum"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetButtonIdNameFromEnum")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetButtonIdNameFromEnum {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetButtonIdNameFromEnum")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetButtonIdNameFromEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetButtonIdNameFromEnum")]
impl crate::OVR::OpenVR::IVRSystem__GetButtonIdNameFromEnum {
    pub fn BeginInvoke(
        &mut self,
        eButtonId: crate::OVR::OpenVR::EVRButtonId,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (eButtonId, callback, object))?;
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
        eButtonId: crate::OVR::OpenVR::EVRButtonId,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (eButtonId))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetButtonIdNameFromEnum")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetButtonIdNameFromEnum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerAxisTypeNameFromEnum")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetControllerAxisTypeNameFromEnum {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerAxisTypeNameFromEnum")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetControllerAxisTypeNameFromEnum => "OVR.OpenVR"
    ."IVRSystem/_GetControllerAxisTypeNameFromEnum"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerAxisTypeNameFromEnum")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSystem__GetControllerAxisTypeNameFromEnum {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerAxisTypeNameFromEnum")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetControllerAxisTypeNameFromEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerAxisTypeNameFromEnum")]
impl crate::OVR::OpenVR::IVRSystem__GetControllerAxisTypeNameFromEnum {
    pub fn BeginInvoke(
        &mut self,
        eAxisType: crate::OVR::OpenVR::EVRControllerAxisType,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (eAxisType, callback, object))?;
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
        eAxisType: crate::OVR::OpenVR::EVRControllerAxisType,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (eAxisType))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerAxisTypeNameFromEnum")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetControllerAxisTypeNameFromEnum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerRoleForTrackedDeviceIndex")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetControllerRoleForTrackedDeviceIndex {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerRoleForTrackedDeviceIndex")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetControllerRoleForTrackedDeviceIndex => "OVR.OpenVR"
    ."IVRSystem/_GetControllerRoleForTrackedDeviceIndex"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerRoleForTrackedDeviceIndex")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSystem__GetControllerRoleForTrackedDeviceIndex {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerRoleForTrackedDeviceIndex")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetControllerRoleForTrackedDeviceIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerRoleForTrackedDeviceIndex")]
impl crate::OVR::OpenVR::IVRSystem__GetControllerRoleForTrackedDeviceIndex {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (unDeviceIndex, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackedControllerRole> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::ETrackedControllerRole = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackedControllerRole> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::ETrackedControllerRole = __cordl_object
            .invoke("Invoke", (unDeviceIndex))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerRoleForTrackedDeviceIndex")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetControllerRoleForTrackedDeviceIndex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerState")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetControllerState {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetControllerState =>
    "OVR.OpenVR"."IVRSystem/_GetControllerState"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerState")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetControllerState {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerState")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetControllerState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerState")]
impl crate::OVR::OpenVR::IVRSystem__GetControllerState {
    pub fn BeginInvoke(
        &mut self,
        unControllerDeviceIndex: u32,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t,
        >,
        unControllerStateSize: u32,
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
                    unControllerDeviceIndex,
                    pControllerState,
                    unControllerStateSize,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pControllerState, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unControllerDeviceIndex: u32,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t,
        >,
        unControllerStateSize: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Invoke",
                (unControllerDeviceIndex, pControllerState, unControllerStateSize),
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerState")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetControllerState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerStateWithPose")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetControllerStateWithPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerStateWithPose")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetControllerStateWithPose => "OVR.OpenVR"
    ."IVRSystem/_GetControllerStateWithPose"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerStateWithPose")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetControllerStateWithPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerStateWithPose")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetControllerStateWithPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerStateWithPose")]
impl crate::OVR::OpenVR::IVRSystem__GetControllerStateWithPose {
    pub fn BeginInvoke(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        unControllerDeviceIndex: u32,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t,
        >,
        unControllerStateSize: u32,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
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
                    eOrigin,
                    unControllerDeviceIndex,
                    pControllerState,
                    unControllerStateSize,
                    pTrackedDevicePose,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t,
        >,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pControllerState, pTrackedDevicePose, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        unControllerDeviceIndex: u32,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t,
        >,
        unControllerStateSize: u32,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Invoke",
                (
                    eOrigin,
                    unControllerDeviceIndex,
                    pControllerState,
                    unControllerStateSize,
                    pTrackedDevicePose,
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetControllerStateWithPose")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetControllerStateWithPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetD3D9AdapterIndex")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetD3D9AdapterIndex {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetD3D9AdapterIndex")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetD3D9AdapterIndex =>
    "OVR.OpenVR"."IVRSystem/_GetD3D9AdapterIndex"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetD3D9AdapterIndex")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetD3D9AdapterIndex {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetD3D9AdapterIndex")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetD3D9AdapterIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetD3D9AdapterIndex")]
impl crate::OVR::OpenVR::IVRSystem__GetD3D9AdapterIndex {
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
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Invoke", ())?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetD3D9AdapterIndex")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetD3D9AdapterIndex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDXGIOutputInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetDXGIOutputInfo {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDXGIOutputInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetDXGIOutputInfo =>
    "OVR.OpenVR"."IVRSystem/_GetDXGIOutputInfo"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDXGIOutputInfo")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetDXGIOutputInfo {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDXGIOutputInfo")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetDXGIOutputInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDXGIOutputInfo")]
impl crate::OVR::OpenVR::IVRSystem__GetDXGIOutputInfo {
    pub fn BeginInvoke(
        &mut self,
        pnAdapterIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pnAdapterIndex, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pnAdapterIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pnAdapterIndex, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pnAdapterIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pnAdapterIndex))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDXGIOutputInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetDXGIOutputInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDeviceToAbsoluteTrackingPose")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetDeviceToAbsoluteTrackingPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDeviceToAbsoluteTrackingPose")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetDeviceToAbsoluteTrackingPose => "OVR.OpenVR"
    ."IVRSystem/_GetDeviceToAbsoluteTrackingPose"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDeviceToAbsoluteTrackingPose")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetDeviceToAbsoluteTrackingPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDeviceToAbsoluteTrackingPose")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetDeviceToAbsoluteTrackingPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDeviceToAbsoluteTrackingPose")]
impl crate::OVR::OpenVR::IVRSystem__GetDeviceToAbsoluteTrackingPose {
    pub fn BeginInvoke(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        fPredictedSecondsToPhotonsFromNow: f32,
        pTrackedDevicePoseArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::OVR::OpenVR::TrackedDevicePose_t,
                >,
            >,
        >,
        unTrackedDevicePoseArrayCount: u32,
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
                    eOrigin,
                    fPredictedSecondsToPhotonsFromNow,
                    pTrackedDevicePoseArray,
                    unTrackedDevicePoseArrayCount,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        fPredictedSecondsToPhotonsFromNow: f32,
        pTrackedDevicePoseArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::OVR::OpenVR::TrackedDevicePose_t,
                >,
            >,
        >,
        unTrackedDevicePoseArrayCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Invoke",
                (
                    eOrigin,
                    fPredictedSecondsToPhotonsFromNow,
                    pTrackedDevicePoseArray,
                    unTrackedDevicePoseArrayCount,
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetDeviceToAbsoluteTrackingPose")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetDeviceToAbsoluteTrackingPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEventTypeNameFromEnum")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetEventTypeNameFromEnum {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEventTypeNameFromEnum")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetEventTypeNameFromEnum
    => "OVR.OpenVR"."IVRSystem/_GetEventTypeNameFromEnum"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEventTypeNameFromEnum")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetEventTypeNameFromEnum {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEventTypeNameFromEnum")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetEventTypeNameFromEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEventTypeNameFromEnum")]
impl crate::OVR::OpenVR::IVRSystem__GetEventTypeNameFromEnum {
    pub fn BeginInvoke(
        &mut self,
        eType: crate::OVR::OpenVR::EVREventType,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (eType, callback, object))?;
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
        eType: crate::OVR::OpenVR::EVREventType,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (eType))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEventTypeNameFromEnum")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetEventTypeNameFromEnum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEyeToHeadTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetEyeToHeadTransform {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEyeToHeadTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetEyeToHeadTransform =>
    "OVR.OpenVR"."IVRSystem/_GetEyeToHeadTransform"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEyeToHeadTransform")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetEyeToHeadTransform {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEyeToHeadTransform")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetEyeToHeadTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEyeToHeadTransform")]
impl crate::OVR::OpenVR::IVRSystem__GetEyeToHeadTransform {
    pub fn BeginInvoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (eEye, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
            .invoke("Invoke", (eEye))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetEyeToHeadTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetEyeToHeadTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetFloatTrackedDeviceProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetFloatTrackedDeviceProperty {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetFloatTrackedDeviceProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetFloatTrackedDeviceProperty => "OVR.OpenVR"
    ."IVRSystem/_GetFloatTrackedDeviceProperty"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetFloatTrackedDeviceProperty")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetFloatTrackedDeviceProperty {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetFloatTrackedDeviceProperty")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetFloatTrackedDeviceProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetFloatTrackedDeviceProperty")]
impl crate::OVR::OpenVR::IVRSystem__GetFloatTrackedDeviceProperty {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
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
            .invoke("BeginInvoke", (unDeviceIndex, prop, pError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("Invoke", (unDeviceIndex, prop, pError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetFloatTrackedDeviceProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetFloatTrackedDeviceProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetHiddenAreaMesh")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetHiddenAreaMesh {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetHiddenAreaMesh")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetHiddenAreaMesh =>
    "OVR.OpenVR"."IVRSystem/_GetHiddenAreaMesh"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetHiddenAreaMesh")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetHiddenAreaMesh {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetHiddenAreaMesh")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetHiddenAreaMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetHiddenAreaMesh")]
impl crate::OVR::OpenVR::IVRSystem__GetHiddenAreaMesh {
    pub fn BeginInvoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        _cordl_type: crate::OVR::OpenVR::EHiddenAreaMeshType,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (eEye, _cordl_type, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HiddenAreaMesh_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HiddenAreaMesh_t = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        _cordl_type: crate::OVR::OpenVR::EHiddenAreaMeshType,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HiddenAreaMesh_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HiddenAreaMesh_t = __cordl_object
            .invoke("Invoke", (eEye, _cordl_type))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetHiddenAreaMesh")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetHiddenAreaMesh {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetInt32TrackedDeviceProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetInt32TrackedDeviceProperty {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetInt32TrackedDeviceProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetInt32TrackedDeviceProperty => "OVR.OpenVR"
    ."IVRSystem/_GetInt32TrackedDeviceProperty"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetInt32TrackedDeviceProperty")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetInt32TrackedDeviceProperty {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetInt32TrackedDeviceProperty")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetInt32TrackedDeviceProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetInt32TrackedDeviceProperty")]
impl crate::OVR::OpenVR::IVRSystem__GetInt32TrackedDeviceProperty {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
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
            .invoke("BeginInvoke", (unDeviceIndex, prop, pError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Invoke", (unDeviceIndex, prop, pError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetInt32TrackedDeviceProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetInt32TrackedDeviceProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetMatrix34TrackedDeviceProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetMatrix34TrackedDeviceProperty {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetMatrix34TrackedDeviceProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetMatrix34TrackedDeviceProperty => "OVR.OpenVR"
    ."IVRSystem/_GetMatrix34TrackedDeviceProperty"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetMatrix34TrackedDeviceProperty")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSystem__GetMatrix34TrackedDeviceProperty {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetMatrix34TrackedDeviceProperty")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetMatrix34TrackedDeviceProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetMatrix34TrackedDeviceProperty")]
impl crate::OVR::OpenVR::IVRSystem__GetMatrix34TrackedDeviceProperty {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
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
            .invoke("BeginInvoke", (unDeviceIndex, prop, pError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
            .invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
            .invoke("Invoke", (unDeviceIndex, prop, pError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetMatrix34TrackedDeviceProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetMatrix34TrackedDeviceProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetOutputDevice")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetOutputDevice {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetOutputDevice")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetOutputDevice =>
    "OVR.OpenVR"."IVRSystem/_GetOutputDevice"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetOutputDevice")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetOutputDevice {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetOutputDevice")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetOutputDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetOutputDevice")]
impl crate::OVR::OpenVR::IVRSystem__GetOutputDevice {
    pub fn BeginInvoke(
        &mut self,
        pnDevice: quest_hook::libil2cpp::ByRefMut<u64>,
        textureType: crate::OVR::OpenVR::ETextureType,
        pInstance: crate::System::IntPtr,
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
                (pnDevice, textureType, pInstance, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pnDevice: quest_hook::libil2cpp::ByRefMut<u64>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pnDevice, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pnDevice: quest_hook::libil2cpp::ByRefMut<u64>,
        textureType: crate::OVR::OpenVR::ETextureType,
        pInstance: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pnDevice, textureType, pInstance))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetOutputDevice")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetOutputDevice {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionMatrix")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetProjectionMatrix {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionMatrix")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetProjectionMatrix =>
    "OVR.OpenVR"."IVRSystem/_GetProjectionMatrix"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionMatrix")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetProjectionMatrix {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionMatrix")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetProjectionMatrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionMatrix")]
impl crate::OVR::OpenVR::IVRSystem__GetProjectionMatrix {
    pub fn BeginInvoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        fNearZ: f32,
        fFarZ: f32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (eEye, fNearZ, fFarZ, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix44_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix44_t = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        fNearZ: f32,
        fFarZ: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix44_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix44_t = __cordl_object
            .invoke("Invoke", (eEye, fNearZ, fFarZ))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionMatrix")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetProjectionMatrix {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionRaw")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetProjectionRaw {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionRaw")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetProjectionRaw =>
    "OVR.OpenVR"."IVRSystem/_GetProjectionRaw"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionRaw")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetProjectionRaw {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionRaw")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetProjectionRaw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionRaw")]
impl crate::OVR::OpenVR::IVRSystem__GetProjectionRaw {
    pub fn BeginInvoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pfLeft: quest_hook::libil2cpp::ByRefMut<f32>,
        pfRight: quest_hook::libil2cpp::ByRefMut<f32>,
        pfTop: quest_hook::libil2cpp::ByRefMut<f32>,
        pfBottom: quest_hook::libil2cpp::ByRefMut<f32>,
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
                (eEye, pfLeft, pfRight, pfTop, pfBottom, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pfLeft: quest_hook::libil2cpp::ByRefMut<f32>,
        pfRight: quest_hook::libil2cpp::ByRefMut<f32>,
        pfTop: quest_hook::libil2cpp::ByRefMut<f32>,
        pfBottom: quest_hook::libil2cpp::ByRefMut<f32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pfLeft, pfRight, pfTop, pfBottom, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pfLeft: quest_hook::libil2cpp::ByRefMut<f32>,
        pfRight: quest_hook::libil2cpp::ByRefMut<f32>,
        pfTop: quest_hook::libil2cpp::ByRefMut<f32>,
        pfBottom: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (eEye, pfLeft, pfRight, pfTop, pfBottom))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetProjectionRaw")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetProjectionRaw {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetPropErrorNameFromEnum")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetPropErrorNameFromEnum {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetPropErrorNameFromEnum")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetPropErrorNameFromEnum
    => "OVR.OpenVR"."IVRSystem/_GetPropErrorNameFromEnum"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetPropErrorNameFromEnum")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetPropErrorNameFromEnum {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetPropErrorNameFromEnum")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetPropErrorNameFromEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetPropErrorNameFromEnum")]
impl crate::OVR::OpenVR::IVRSystem__GetPropErrorNameFromEnum {
    pub fn BeginInvoke(
        &mut self,
        error: crate::OVR::OpenVR::ETrackedPropertyError,
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
        error: crate::OVR::OpenVR::ETrackedPropertyError,
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetPropErrorNameFromEnum")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetPropErrorNameFromEnum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRawZeroPoseToStandingAbsoluteTrackingPose")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetRawZeroPoseToStandingAbsoluteTrackingPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRawZeroPoseToStandingAbsoluteTrackingPose")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetRawZeroPoseToStandingAbsoluteTrackingPose =>
    "OVR.OpenVR"."IVRSystem/_GetRawZeroPoseToStandingAbsoluteTrackingPose"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRawZeroPoseToStandingAbsoluteTrackingPose")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSystem__GetRawZeroPoseToStandingAbsoluteTrackingPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRawZeroPoseToStandingAbsoluteTrackingPose")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetRawZeroPoseToStandingAbsoluteTrackingPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRawZeroPoseToStandingAbsoluteTrackingPose")]
impl crate::OVR::OpenVR::IVRSystem__GetRawZeroPoseToStandingAbsoluteTrackingPose {
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
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRawZeroPoseToStandingAbsoluteTrackingPose")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetRawZeroPoseToStandingAbsoluteTrackingPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRecommendedRenderTargetSize")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetRecommendedRenderTargetSize {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRecommendedRenderTargetSize")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetRecommendedRenderTargetSize => "OVR.OpenVR"
    ."IVRSystem/_GetRecommendedRenderTargetSize"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRecommendedRenderTargetSize")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetRecommendedRenderTargetSize {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRecommendedRenderTargetSize")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetRecommendedRenderTargetSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRecommendedRenderTargetSize")]
impl crate::OVR::OpenVR::IVRSystem__GetRecommendedRenderTargetSize {
    pub fn BeginInvoke(
        &mut self,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pnWidth, pnHeight, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pnWidth, pnHeight, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pnWidth, pnHeight))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetRecommendedRenderTargetSize")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetRecommendedRenderTargetSize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSeatedZeroPoseToStandingAbsoluteTrackingPose")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetSeatedZeroPoseToStandingAbsoluteTrackingPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSeatedZeroPoseToStandingAbsoluteTrackingPose")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetSeatedZeroPoseToStandingAbsoluteTrackingPose =>
    "OVR.OpenVR"."IVRSystem/_GetSeatedZeroPoseToStandingAbsoluteTrackingPose"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSeatedZeroPoseToStandingAbsoluteTrackingPose")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSystem__GetSeatedZeroPoseToStandingAbsoluteTrackingPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSeatedZeroPoseToStandingAbsoluteTrackingPose")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetSeatedZeroPoseToStandingAbsoluteTrackingPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSeatedZeroPoseToStandingAbsoluteTrackingPose")]
impl crate::OVR::OpenVR::IVRSystem__GetSeatedZeroPoseToStandingAbsoluteTrackingPose {
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
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSeatedZeroPoseToStandingAbsoluteTrackingPose")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetSeatedZeroPoseToStandingAbsoluteTrackingPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSortedTrackedDeviceIndicesOfClass")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetSortedTrackedDeviceIndicesOfClass {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSortedTrackedDeviceIndicesOfClass")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetSortedTrackedDeviceIndicesOfClass => "OVR.OpenVR"
    ."IVRSystem/_GetSortedTrackedDeviceIndicesOfClass"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSortedTrackedDeviceIndicesOfClass")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSystem__GetSortedTrackedDeviceIndicesOfClass {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSortedTrackedDeviceIndicesOfClass")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetSortedTrackedDeviceIndicesOfClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSortedTrackedDeviceIndicesOfClass")]
impl crate::OVR::OpenVR::IVRSystem__GetSortedTrackedDeviceIndicesOfClass {
    pub fn BeginInvoke(
        &mut self,
        eTrackedDeviceClass: crate::OVR::OpenVR::ETrackedDeviceClass,
        punTrackedDeviceIndexArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        >,
        unTrackedDeviceIndexArrayCount: u32,
        unRelativeToTrackedDeviceIndex: u32,
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
                    eTrackedDeviceClass,
                    punTrackedDeviceIndexArray,
                    unTrackedDeviceIndexArrayCount,
                    unRelativeToTrackedDeviceIndex,
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
        eTrackedDeviceClass: crate::OVR::OpenVR::ETrackedDeviceClass,
        punTrackedDeviceIndexArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        >,
        unTrackedDeviceIndexArrayCount: u32,
        unRelativeToTrackedDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "Invoke",
                (
                    eTrackedDeviceClass,
                    punTrackedDeviceIndexArray,
                    unTrackedDeviceIndexArrayCount,
                    unRelativeToTrackedDeviceIndex,
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetSortedTrackedDeviceIndicesOfClass")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetSortedTrackedDeviceIndicesOfClass {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetStringTrackedDeviceProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetStringTrackedDeviceProperty {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetStringTrackedDeviceProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetStringTrackedDeviceProperty => "OVR.OpenVR"
    ."IVRSystem/_GetStringTrackedDeviceProperty"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetStringTrackedDeviceProperty")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetStringTrackedDeviceProperty {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetStringTrackedDeviceProperty")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetStringTrackedDeviceProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetStringTrackedDeviceProperty")]
impl crate::OVR::OpenVR::IVRSystem__GetStringTrackedDeviceProperty {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
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
                (unDeviceIndex, prop, pchValue, unBufferSize, pError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("Invoke", (unDeviceIndex, prop, pchValue, unBufferSize, pError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetStringTrackedDeviceProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetStringTrackedDeviceProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTimeSinceLastVsync")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetTimeSinceLastVsync {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTimeSinceLastVsync")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetTimeSinceLastVsync =>
    "OVR.OpenVR"."IVRSystem/_GetTimeSinceLastVsync"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTimeSinceLastVsync")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetTimeSinceLastVsync {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTimeSinceLastVsync")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetTimeSinceLastVsync {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTimeSinceLastVsync")]
impl crate::OVR::OpenVR::IVRSystem__GetTimeSinceLastVsync {
    pub fn BeginInvoke(
        &mut self,
        pfSecondsSinceLastVsync: quest_hook::libil2cpp::ByRefMut<f32>,
        pulFrameCounter: quest_hook::libil2cpp::ByRefMut<u64>,
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
                (pfSecondsSinceLastVsync, pulFrameCounter, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pfSecondsSinceLastVsync: quest_hook::libil2cpp::ByRefMut<f32>,
        pulFrameCounter: quest_hook::libil2cpp::ByRefMut<u64>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pfSecondsSinceLastVsync, pulFrameCounter, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pfSecondsSinceLastVsync: quest_hook::libil2cpp::ByRefMut<f32>,
        pulFrameCounter: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pfSecondsSinceLastVsync, pulFrameCounter))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTimeSinceLastVsync")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetTimeSinceLastVsync {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceActivityLevel")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetTrackedDeviceActivityLevel {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceActivityLevel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetTrackedDeviceActivityLevel => "OVR.OpenVR"
    ."IVRSystem/_GetTrackedDeviceActivityLevel"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceActivityLevel")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceActivityLevel {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceActivityLevel")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceActivityLevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceActivityLevel")]
impl crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceActivityLevel {
    pub fn BeginInvoke(
        &mut self,
        unDeviceId: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (unDeviceId, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EDeviceActivityLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EDeviceActivityLevel = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EDeviceActivityLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EDeviceActivityLevel = __cordl_object
            .invoke("Invoke", (unDeviceId))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceActivityLevel")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceActivityLevel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceClass")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetTrackedDeviceClass {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceClass")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__GetTrackedDeviceClass =>
    "OVR.OpenVR"."IVRSystem/_GetTrackedDeviceClass"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceClass")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceClass {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceClass")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceClass")]
impl crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceClass {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (unDeviceIndex, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackedDeviceClass> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::ETrackedDeviceClass = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackedDeviceClass> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::ETrackedDeviceClass = __cordl_object
            .invoke("Invoke", (unDeviceIndex))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceClass")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceClass {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceIndexForControllerRole")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetTrackedDeviceIndexForControllerRole {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceIndexForControllerRole")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetTrackedDeviceIndexForControllerRole => "OVR.OpenVR"
    ."IVRSystem/_GetTrackedDeviceIndexForControllerRole"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceIndexForControllerRole")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceIndexForControllerRole {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceIndexForControllerRole")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceIndexForControllerRole {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceIndexForControllerRole")]
impl crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceIndexForControllerRole {
    pub fn BeginInvoke(
        &mut self,
        unDeviceType: crate::OVR::OpenVR::ETrackedControllerRole,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (unDeviceType, callback, object))?;
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
        unDeviceType: crate::OVR::OpenVR::ETrackedControllerRole,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("Invoke", (unDeviceType))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetTrackedDeviceIndexForControllerRole")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetTrackedDeviceIndexForControllerRole {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetUint64TrackedDeviceProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__GetUint64TrackedDeviceProperty {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetUint64TrackedDeviceProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__GetUint64TrackedDeviceProperty => "OVR.OpenVR"
    ."IVRSystem/_GetUint64TrackedDeviceProperty"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetUint64TrackedDeviceProperty")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__GetUint64TrackedDeviceProperty {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetUint64TrackedDeviceProperty")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__GetUint64TrackedDeviceProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetUint64TrackedDeviceProperty")]
impl crate::OVR::OpenVR::IVRSystem__GetUint64TrackedDeviceProperty {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
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
            .invoke("BeginInvoke", (unDeviceIndex, prop, pError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object
            .invoke("Invoke", (unDeviceIndex, prop, pError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_GetUint64TrackedDeviceProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__GetUint64TrackedDeviceProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsDisplayOnDesktop")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__IsDisplayOnDesktop {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsDisplayOnDesktop")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__IsDisplayOnDesktop =>
    "OVR.OpenVR"."IVRSystem/_IsDisplayOnDesktop"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsDisplayOnDesktop")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__IsDisplayOnDesktop {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsDisplayOnDesktop")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__IsDisplayOnDesktop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsDisplayOnDesktop")]
impl crate::OVR::OpenVR::IVRSystem__IsDisplayOnDesktop {
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsDisplayOnDesktop")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__IsDisplayOnDesktop {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsInputAvailable")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__IsInputAvailable {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsInputAvailable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__IsInputAvailable =>
    "OVR.OpenVR"."IVRSystem/_IsInputAvailable"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsInputAvailable")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__IsInputAvailable {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsInputAvailable")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__IsInputAvailable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsInputAvailable")]
impl crate::OVR::OpenVR::IVRSystem__IsInputAvailable {
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsInputAvailable")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__IsInputAvailable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsSteamVRDrawingControllers")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__IsSteamVRDrawingControllers {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsSteamVRDrawingControllers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__IsSteamVRDrawingControllers => "OVR.OpenVR"
    ."IVRSystem/_IsSteamVRDrawingControllers"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsSteamVRDrawingControllers")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__IsSteamVRDrawingControllers {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsSteamVRDrawingControllers")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__IsSteamVRDrawingControllers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsSteamVRDrawingControllers")]
impl crate::OVR::OpenVR::IVRSystem__IsSteamVRDrawingControllers {
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsSteamVRDrawingControllers")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__IsSteamVRDrawingControllers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsTrackedDeviceConnected")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__IsTrackedDeviceConnected {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsTrackedDeviceConnected")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__IsTrackedDeviceConnected
    => "OVR.OpenVR"."IVRSystem/_IsTrackedDeviceConnected"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsTrackedDeviceConnected")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__IsTrackedDeviceConnected {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsTrackedDeviceConnected")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__IsTrackedDeviceConnected {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsTrackedDeviceConnected")]
impl crate::OVR::OpenVR::IVRSystem__IsTrackedDeviceConnected {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (unDeviceIndex, callback, object))?;
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
    pub fn Invoke(&mut self, unDeviceIndex: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (unDeviceIndex))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_IsTrackedDeviceConnected")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__IsTrackedDeviceConnected {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PerformFirmwareUpdate")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__PerformFirmwareUpdate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PerformFirmwareUpdate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__PerformFirmwareUpdate =>
    "OVR.OpenVR"."IVRSystem/_PerformFirmwareUpdate"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PerformFirmwareUpdate")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__PerformFirmwareUpdate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PerformFirmwareUpdate")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__PerformFirmwareUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PerformFirmwareUpdate")]
impl crate::OVR::OpenVR::IVRSystem__PerformFirmwareUpdate {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (unDeviceIndex, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRFirmwareError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRFirmwareError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRFirmwareError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRFirmwareError = __cordl_object
            .invoke("Invoke", (unDeviceIndex))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PerformFirmwareUpdate")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__PerformFirmwareUpdate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__PollNextEvent {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__PollNextEvent =>
    "OVR.OpenVR"."IVRSystem/_PollNextEvent"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEvent")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__PollNextEvent {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEvent")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__PollNextEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEvent")]
impl crate::OVR::OpenVR::IVRSystem__PollNextEvent {
    pub fn BeginInvoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pEvent, uncbVREvent, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (pEvent, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (pEvent, uncbVREvent))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRSystem__PollNextEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEventWithPose")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__PollNextEventWithPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEventWithPose")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__PollNextEventWithPose =>
    "OVR.OpenVR"."IVRSystem/_PollNextEventWithPose"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEventWithPose")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__PollNextEventWithPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEventWithPose")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__PollNextEventWithPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEventWithPose")]
impl crate::OVR::OpenVR::IVRSystem__PollNextEventWithPose {
    pub fn BeginInvoke(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
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
                (eOrigin, pEvent, uncbVREvent, pTrackedDevicePose, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pEvent, pTrackedDevicePose, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (eOrigin, pEvent, uncbVREvent, pTrackedDevicePose))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_PollNextEventWithPose")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__PollNextEventWithPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ResetSeatedZeroPose")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__ResetSeatedZeroPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ResetSeatedZeroPose")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__ResetSeatedZeroPose =>
    "OVR.OpenVR"."IVRSystem/_ResetSeatedZeroPose"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ResetSeatedZeroPose")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__ResetSeatedZeroPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ResetSeatedZeroPose")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__ResetSeatedZeroPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ResetSeatedZeroPose")]
impl crate::OVR::OpenVR::IVRSystem__ResetSeatedZeroPose {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ResetSeatedZeroPose")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__ResetSeatedZeroPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_SetDisplayVisibility")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__SetDisplayVisibility {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_SetDisplayVisibility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__SetDisplayVisibility =>
    "OVR.OpenVR"."IVRSystem/_SetDisplayVisibility"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_SetDisplayVisibility")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__SetDisplayVisibility {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_SetDisplayVisibility")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__SetDisplayVisibility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_SetDisplayVisibility")]
impl crate::OVR::OpenVR::IVRSystem__SetDisplayVisibility {
    pub fn BeginInvoke(
        &mut self,
        bIsVisibleOnDesktop: bool,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (bIsVisibleOnDesktop, callback, object))?;
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
        bIsVisibleOnDesktop: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (bIsVisibleOnDesktop))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_SetDisplayVisibility")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__SetDisplayVisibility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationPause")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__ShouldApplicationPause {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationPause")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__ShouldApplicationPause
    => "OVR.OpenVR"."IVRSystem/_ShouldApplicationPause"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationPause")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__ShouldApplicationPause {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationPause")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__ShouldApplicationPause {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationPause")]
impl crate::OVR::OpenVR::IVRSystem__ShouldApplicationPause {
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationPause")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__ShouldApplicationPause {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationReduceRenderingWork")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__ShouldApplicationReduceRenderingWork {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationReduceRenderingWork")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSystem__ShouldApplicationReduceRenderingWork => "OVR.OpenVR"
    ."IVRSystem/_ShouldApplicationReduceRenderingWork"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationReduceRenderingWork")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSystem__ShouldApplicationReduceRenderingWork {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationReduceRenderingWork")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSystem__ShouldApplicationReduceRenderingWork {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationReduceRenderingWork")]
impl crate::OVR::OpenVR::IVRSystem__ShouldApplicationReduceRenderingWork {
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_ShouldApplicationReduceRenderingWork")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__ShouldApplicationReduceRenderingWork {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_TriggerHapticPulse")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSystem__TriggerHapticPulse {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_TriggerHapticPulse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSystem__TriggerHapticPulse =>
    "OVR.OpenVR"."IVRSystem/_TriggerHapticPulse"
);
#[cfg(feature = "OVR+OpenVR+IVRSystem+_TriggerHapticPulse")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSystem__TriggerHapticPulse {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_TriggerHapticPulse")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSystem__TriggerHapticPulse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSystem+_TriggerHapticPulse")]
impl crate::OVR::OpenVR::IVRSystem__TriggerHapticPulse {
    pub fn BeginInvoke(
        &mut self,
        unControllerDeviceIndex: u32,
        unAxisId: u32,
        usDurationMicroSec: u16,
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
                (unControllerDeviceIndex, unAxisId, usDurationMicroSec, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unControllerDeviceIndex: u32,
        unAxisId: u32,
        usDurationMicroSec: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (unControllerDeviceIndex, unAxisId, usDurationMicroSec))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSystem+_TriggerHapticPulse")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSystem__TriggerHapticPulse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
