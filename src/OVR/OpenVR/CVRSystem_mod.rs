#[cfg(feature = "OVR+OpenVR+CVRSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRSystem,
}
#[cfg(feature = "OVR+OpenVR+CVRSystem")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRSystem {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRSystem";
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
#[cfg(feature = "OVR+OpenVR+CVRSystem")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem")]
impl crate::OVR::OpenVR::CVRSystem {
    #[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateUnion")]
    pub type GetControllerStateUnion = crate::OVR::OpenVR::CVRSystem_GetControllerStateUnion;
    #[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateWithPoseUnion")]
    pub type GetControllerStateWithPoseUnion = crate::OVR::OpenVR::CVRSystem_GetControllerStateWithPoseUnion;
    #[cfg(feature = "OVR+OpenVR+CVRSystem+PollNextEventUnion")]
    pub type PollNextEventUnion = crate::OVR::OpenVR::CVRSystem_PollNextEventUnion;
    #[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStatePacked")]
    pub type _GetControllerStatePacked = crate::OVR::OpenVR::CVRSystem__GetControllerStatePacked;
    #[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStateWithPosePacked")]
    pub type _GetControllerStateWithPosePacked = crate::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked;
    #[cfg(feature = "OVR+OpenVR+CVRSystem+_PollNextEventPacked")]
    pub type _PollNextEventPacked = crate::OVR::OpenVR::CVRSystem__PollNextEventPacked;
    pub fn AcknowledgeQuit_Exiting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("AcknowledgeQuit_Exiting")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "AcknowledgeQuit_Exiting", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AcknowledgeQuit_UserPrompt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("AcknowledgeQuit_UserPrompt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "AcknowledgeQuit_UserPrompt", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyTransform(
        &mut self,
        pOutputPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        pTransform: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::TrackedDevicePose_t,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::TrackedDevicePose_t,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ApplyTransform")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "ApplyTransform", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pOutputPose, pTrackedDevicePose, pTransform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeDistortion(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        fU: f32,
        fV: f32,
        pDistortionCoordinates: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::DistortionCoordinates_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::OVR::OpenVR::EVREye,
                    f32,
                    f32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::DistortionCoordinates_t,
                    >,
                ),
                bool,
                4usize,
            >("ComputeDistortion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "ComputeDistortion", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (eEye, fU, fV, pDistortionCoordinates))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DriverDebugRequest(
        &mut self,
        unDeviceIndex: u32,
        pchRequest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchResponseBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unResponseBufferSize: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                ),
                u32,
                4usize,
            >("DriverDebugRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "DriverDebugRequest", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (unDeviceIndex, pchRequest, pchResponseBuffer, unResponseBufferSize),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetArrayTrackedDeviceProperty(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::ETrackedDeviceProperty,
                    u32,
                    crate::System::IntPtr,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::ETrackedPropertyError,
                    >,
                ),
                u32,
                6usize,
            >("GetArrayTrackedDeviceProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetArrayTrackedDeviceProperty", 6usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (unDeviceIndex, prop, propType, pBuffer, unBufferSize, pError),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBoolTrackedDeviceProperty(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::ETrackedDeviceProperty,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::ETrackedPropertyError,
                    >,
                ),
                bool,
                3usize,
            >("GetBoolTrackedDeviceProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetBoolTrackedDeviceProperty", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (unDeviceIndex, prop, pError))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonIdNameFromEnum(
        &mut self,
        eButtonId: crate::OVR::OpenVR::EVRButtonId,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EVRButtonId),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetButtonIdNameFromEnum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetButtonIdNameFromEnum", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (eButtonId))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerAxisTypeNameFromEnum(
        &mut self,
        eAxisType: crate::OVR::OpenVR::EVRControllerAxisType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EVRControllerAxisType),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetControllerAxisTypeNameFromEnum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetControllerAxisTypeNameFromEnum", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (eAxisType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerRoleForTrackedDeviceIndex(
        &mut self,
        unDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackedControllerRole> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32),
                crate::OVR::OpenVR::ETrackedControllerRole,
                1usize,
            >("GetControllerRoleForTrackedDeviceIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetControllerRoleForTrackedDeviceIndex", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::ETrackedControllerRole = unsafe {
            method.invoke_unchecked(self, (unDeviceIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerState(
        &mut self,
        unControllerDeviceIndex: u32,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t,
        >,
        unControllerStateSize: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VRControllerState_t,
                    >,
                    u32,
                ),
                bool,
                3usize,
            >("GetControllerState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetControllerState", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (unControllerDeviceIndex, pControllerState, unControllerStateSize),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerStateWithPose(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::OVR::OpenVR::ETrackingUniverseOrigin,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VRControllerState_t,
                    >,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::TrackedDevicePose_t,
                    >,
                ),
                bool,
                5usize,
            >("GetControllerStateWithPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetControllerStateWithPose", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        eOrigin,
                        unControllerDeviceIndex,
                        pControllerState,
                        unControllerStateSize,
                        pTrackedDevicePose,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetD3D9AdapterIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetD3D9AdapterIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetD3D9AdapterIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDXGIOutputInfo(
        &mut self,
        pnAdapterIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<i32>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetDXGIOutputInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetDXGIOutputInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pnAdapterIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDeviceToAbsoluteTrackingPose(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        fPredictedSecondsToPhotonsFromNow: f32,
        pTrackedDevicePoseArray: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::TrackedDevicePose_t>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::OVR::OpenVR::ETrackingUniverseOrigin,
                    f32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::OVR::OpenVR::TrackedDevicePose_t,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("GetDeviceToAbsoluteTrackingPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetDeviceToAbsoluteTrackingPose", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (eOrigin, fPredictedSecondsToPhotonsFromNow, pTrackedDevicePoseArray),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEventTypeNameFromEnum(
        &mut self,
        eType: crate::OVR::OpenVR::EVREventType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EVREventType),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetEventTypeNameFromEnum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetEventTypeNameFromEnum", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (eType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeToHeadTransform(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EVREye),
                crate::OVR::OpenVR::HmdMatrix34_t,
                1usize,
            >("GetEyeToHeadTransform")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetEyeToHeadTransform", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = unsafe {
            method.invoke_unchecked(self, (eEye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFloatTrackedDeviceProperty(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::ETrackedDeviceProperty,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::ETrackedPropertyError,
                    >,
                ),
                f32,
                3usize,
            >("GetFloatTrackedDeviceProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetFloatTrackedDeviceProperty", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked(self, (unDeviceIndex, prop, pError))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHiddenAreaMesh(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        _cordl_type: crate::OVR::OpenVR::EHiddenAreaMeshType,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HiddenAreaMesh_t> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EVREye, crate::OVR::OpenVR::EHiddenAreaMeshType),
                crate::OVR::OpenVR::HiddenAreaMesh_t,
                2usize,
            >("GetHiddenAreaMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetHiddenAreaMesh", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::HiddenAreaMesh_t = unsafe {
            method.invoke_unchecked(self, (eEye, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInt32TrackedDeviceProperty(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::ETrackedDeviceProperty,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::ETrackedPropertyError,
                    >,
                ),
                i32,
                3usize,
            >("GetInt32TrackedDeviceProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetInt32TrackedDeviceProperty", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (unDeviceIndex, prop, pError))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMatrix34TrackedDeviceProperty(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::ETrackedDeviceProperty,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::ETrackedPropertyError,
                    >,
                ),
                crate::OVR::OpenVR::HmdMatrix34_t,
                3usize,
            >("GetMatrix34TrackedDeviceProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetMatrix34TrackedDeviceProperty", 3usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = unsafe {
            method.invoke_unchecked(self, (unDeviceIndex, prop, pError))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputDevice(
        &mut self,
        pnDevice: quest_hook::libil2cpp::ByRefMut<u64>,
        textureType: crate::OVR::OpenVR::ETextureType,
        pInstance: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u64>,
                    crate::OVR::OpenVR::ETextureType,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("GetOutputDevice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetOutputDevice", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pnDevice, textureType, pInstance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetProjectionMatrix(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        fNearZ: f32,
        fFarZ: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix44_t> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EVREye, f32, f32),
                crate::OVR::OpenVR::HmdMatrix44_t,
                3usize,
            >("GetProjectionMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetProjectionMatrix", 3usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix44_t = unsafe {
            method.invoke_unchecked(self, (eEye, fNearZ, fFarZ))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetProjectionRaw(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pfLeft: quest_hook::libil2cpp::ByRefMut<f32>,
        pfRight: quest_hook::libil2cpp::ByRefMut<f32>,
        pfTop: quest_hook::libil2cpp::ByRefMut<f32>,
        pfBottom: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::OVR::OpenVR::EVREye,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("GetProjectionRaw")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetProjectionRaw", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eEye, pfLeft, pfRight, pfTop, pfBottom))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropErrorNameFromEnum(
        &mut self,
        error: crate::OVR::OpenVR::ETrackedPropertyError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::ETrackedPropertyError),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetPropErrorNameFromEnum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropErrorNameFromEnum", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (error))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRawZeroPoseToStandingAbsoluteTrackingPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::OVR::OpenVR::HmdMatrix34_t,
                0usize,
            >("GetRawZeroPoseToStandingAbsoluteTrackingPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetRawZeroPoseToStandingAbsoluteTrackingPose", 0usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRecommendedRenderTargetSize(
        &mut self,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    quest_hook::libil2cpp::ByRefMut<u32>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetRecommendedRenderTargetSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetRecommendedRenderTargetSize", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pnWidth, pnHeight))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSeatedZeroPoseToStandingAbsoluteTrackingPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::OVR::OpenVR::HmdMatrix34_t,
                0usize,
            >("GetSeatedZeroPoseToStandingAbsoluteTrackingPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetSeatedZeroPoseToStandingAbsoluteTrackingPose", 0usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSortedTrackedDeviceIndicesOfClass(
        &mut self,
        eTrackedDeviceClass: crate::OVR::OpenVR::ETrackedDeviceClass,
        punTrackedDeviceIndexArray: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        >,
        unRelativeToTrackedDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::OVR::OpenVR::ETrackedDeviceClass,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    u32,
                ),
                u32,
                3usize,
            >("GetSortedTrackedDeviceIndicesOfClass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetSortedTrackedDeviceIndicesOfClass", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        eTrackedDeviceClass,
                        punTrackedDeviceIndexArray,
                        unRelativeToTrackedDeviceIndex,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStringTrackedDeviceProperty(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::ETrackedDeviceProperty,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::ETrackedPropertyError,
                    >,
                ),
                u32,
                5usize,
            >("GetStringTrackedDeviceProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetStringTrackedDeviceProperty", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (unDeviceIndex, prop, pchValue, unBufferSize, pError),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeSinceLastVsync(
        &mut self,
        pfSecondsSinceLastVsync: quest_hook::libil2cpp::ByRefMut<f32>,
        pulFrameCounter: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<f32>,
                    quest_hook::libil2cpp::ByRefMut<u64>,
                ),
                bool,
                2usize,
            >("GetTimeSinceLastVsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetTimeSinceLastVsync", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pfSecondsSinceLastVsync, pulFrameCounter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackedDeviceActivityLevel(
        &mut self,
        unDeviceId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EDeviceActivityLevel> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32),
                crate::OVR::OpenVR::EDeviceActivityLevel,
                1usize,
            >("GetTrackedDeviceActivityLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetTrackedDeviceActivityLevel", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EDeviceActivityLevel = unsafe {
            method.invoke_unchecked(self, (unDeviceId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackedDeviceClass(
        &mut self,
        unDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackedDeviceClass> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32),
                crate::OVR::OpenVR::ETrackedDeviceClass,
                1usize,
            >("GetTrackedDeviceClass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetTrackedDeviceClass", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::ETrackedDeviceClass = unsafe {
            method.invoke_unchecked(self, (unDeviceIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackedDeviceIndexForControllerRole(
        &mut self,
        unDeviceType: crate::OVR::OpenVR::ETrackedControllerRole,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::ETrackedControllerRole),
                u32,
                1usize,
            >("GetTrackedDeviceIndexForControllerRole")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetTrackedDeviceIndexForControllerRole", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (unDeviceType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUint64TrackedDeviceProperty(
        &mut self,
        unDeviceIndex: u32,
        prop: crate::OVR::OpenVR::ETrackedDeviceProperty,
        pError: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackedPropertyError,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::ETrackedDeviceProperty,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::ETrackedPropertyError,
                    >,
                ),
                u64,
                3usize,
            >("GetUint64TrackedDeviceProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "GetUint64TrackedDeviceProperty", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked(self, (unDeviceIndex, prop, pError))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsDisplayOnDesktop(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsDisplayOnDesktop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "IsDisplayOnDesktop", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsInputAvailable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsInputAvailable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "IsInputAvailable", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSteamVRDrawingControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsSteamVRDrawingControllers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "IsSteamVRDrawingControllers", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsTrackedDeviceConnected(
        &mut self,
        unDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), bool, 1usize>("IsTrackedDeviceConnected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "IsTrackedDeviceConnected", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (unDeviceIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn PerformFirmwareUpdate(
        &mut self,
        unDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRFirmwareError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32),
                crate::OVR::OpenVR::EVRFirmwareError,
                1usize,
            >("PerformFirmwareUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "PerformFirmwareUpdate", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRFirmwareError = unsafe {
            method.invoke_unchecked(self, (unDeviceIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PollNextEvent(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>, u32),
                bool,
                2usize,
            >("PollNextEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "PollNextEvent", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pEvent, uncbVREvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PollNextEventWithPose(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::OVR::OpenVR::ETrackingUniverseOrigin,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::TrackedDevicePose_t,
                    >,
                ),
                bool,
                4usize,
            >("PollNextEventWithPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "PollNextEventWithPose", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (eOrigin, pEvent, uncbVREvent, pTrackedDevicePose),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetSeatedZeroPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ResetSeatedZeroPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "ResetSeatedZeroPose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDisplayVisibility(
        &mut self,
        bIsVisibleOnDesktop: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), bool, 1usize>("SetDisplayVisibility")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "SetDisplayVisibility", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (bIsVisibleOnDesktop))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldApplicationPause(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("ShouldApplicationPause")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "ShouldApplicationPause", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldApplicationReduceRenderingWork(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("ShouldApplicationReduceRenderingWork")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "ShouldApplicationReduceRenderingWork", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TriggerHapticPulse(
        &mut self,
        unControllerDeviceIndex: u32,
        unAxisId: u32,
        usDurationMicroSec: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32, u32, u16),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TriggerHapticPulse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), "TriggerHapticPulse", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (unControllerDeviceIndex, unAxisId, usDurationMicroSec),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pInterface))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateUnion")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CVRSystem_GetControllerStateUnion {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateUnion")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::CVRSystem_GetControllerStateUnion {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRSystem/GetControllerStateUnion";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateUnion")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OVR::OpenVR::CVRSystem_GetControllerStateUnion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateUnion")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OVR::OpenVR::CVRSystem_GetControllerStateUnion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateUnion")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OVR::OpenVR::CVRSystem_GetControllerStateUnion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateUnion")]
unsafe impl quest_hook::libil2cpp::Return
for crate::OVR::OpenVR::CVRSystem_GetControllerStateUnion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::CVRSystem_GetControllerStateUnion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateUnion")]
impl crate::OVR::OpenVR::CVRSystem_GetControllerStateUnion {}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateWithPoseUnion")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CVRSystem_GetControllerStateWithPoseUnion {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateWithPoseUnion")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::CVRSystem_GetControllerStateWithPoseUnion {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRSystem/GetControllerStateWithPoseUnion";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateWithPoseUnion")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OVR::OpenVR::CVRSystem_GetControllerStateWithPoseUnion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateWithPoseUnion")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OVR::OpenVR::CVRSystem_GetControllerStateWithPoseUnion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateWithPoseUnion")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OVR::OpenVR::CVRSystem_GetControllerStateWithPoseUnion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateWithPoseUnion")]
unsafe impl quest_hook::libil2cpp::Return
for crate::OVR::OpenVR::CVRSystem_GetControllerStateWithPoseUnion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateWithPoseUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::CVRSystem_GetControllerStateWithPoseUnion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateWithPoseUnion")]
impl crate::OVR::OpenVR::CVRSystem_GetControllerStateWithPoseUnion {}
#[cfg(feature = "OVR+OpenVR+CVRSystem+PollNextEventUnion")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CVRSystem_PollNextEventUnion {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+PollNextEventUnion")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::CVRSystem_PollNextEventUnion {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRSystem/PollNextEventUnion";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+PollNextEventUnion")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OVR::OpenVR::CVRSystem_PollNextEventUnion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+PollNextEventUnion")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OVR::OpenVR::CVRSystem_PollNextEventUnion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+PollNextEventUnion")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OVR::OpenVR::CVRSystem_PollNextEventUnion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+PollNextEventUnion")]
unsafe impl quest_hook::libil2cpp::Return
for crate::OVR::OpenVR::CVRSystem_PollNextEventUnion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+PollNextEventUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::CVRSystem_PollNextEventUnion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+PollNextEventUnion")]
impl crate::OVR::OpenVR::CVRSystem_PollNextEventUnion {}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStatePacked")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRSystem__GetControllerStatePacked {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStatePacked")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::CVRSystem__GetControllerStatePacked {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRSystem/_GetControllerStatePacked";
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
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStatePacked")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRSystem__GetControllerStatePacked {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStatePacked")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRSystem__GetControllerStatePacked {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStatePacked")]
impl crate::OVR::OpenVR::CVRSystem__GetControllerStatePacked {
    pub fn BeginInvoke(
        &mut self,
        unControllerDeviceIndex: u32,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        unControllerStateSize: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__GetControllerStatePacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VRControllerState_t_Packed,
                    >,
                    u32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                5usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__GetControllerStatePacked as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        unControllerDeviceIndex,
                        pControllerState,
                        unControllerStateSize,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__GetControllerStatePacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VRControllerState_t_Packed,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                ),
                bool,
                2usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__GetControllerStatePacked as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pControllerState, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unControllerDeviceIndex: u32,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        unControllerStateSize: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__GetControllerStatePacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VRControllerState_t_Packed,
                    >,
                    u32,
                ),
                bool,
                3usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__GetControllerStatePacked as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (unControllerDeviceIndex, pControllerState, unControllerStateSize),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__GetControllerStatePacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__GetControllerStatePacked as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStatePacked")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::CVRSystem__GetControllerStatePacked {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStateWithPosePacked")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRSystem__GetControllerStateWithPosePacked {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStateWithPosePacked")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRSystem/_GetControllerStateWithPosePacked";
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
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStateWithPosePacked")]
impl std::ops::Deref
for crate::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStateWithPosePacked")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStateWithPosePacked")]
impl crate::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked {
    pub fn BeginInvoke(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        unControllerDeviceIndex: u32,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::OVR::OpenVR::ETrackingUniverseOrigin,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VRControllerState_t_Packed,
                    >,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::TrackedDevicePose_t,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                7usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        eOrigin,
                        unControllerDeviceIndex,
                        pControllerState,
                        unControllerStateSize,
                        pTrackedDevicePose,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VRControllerState_t_Packed,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::TrackedDevicePose_t,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                ),
                bool,
                3usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(self, (pControllerState, pTrackedDevicePose, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        unControllerDeviceIndex: u32,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        unControllerStateSize: u32,
        pTrackedDevicePose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::TrackedDevicePose_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::OVR::OpenVR::ETrackingUniverseOrigin,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VRControllerState_t_Packed,
                    >,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::TrackedDevicePose_t,
                    >,
                ),
                bool,
                5usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        eOrigin,
                        unControllerDeviceIndex,
                        pControllerState,
                        unControllerStateSize,
                        pTrackedDevicePose,
                    ),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_GetControllerStateWithPosePacked")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_PollNextEventPacked")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRSystem__PollNextEventPacked {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_PollNextEventPacked")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::CVRSystem__PollNextEventPacked {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRSystem/_PollNextEventPacked";
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
#[cfg(feature = "OVR+OpenVR+CVRSystem+_PollNextEventPacked")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRSystem__PollNextEventPacked {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_PollNextEventPacked")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRSystem__PollNextEventPacked {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_PollNextEventPacked")]
impl crate::OVR::OpenVR::CVRSystem__PollNextEventPacked {
    pub fn BeginInvoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t_Packed>,
        uncbVREvent: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__PollNextEventPacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VREvent_t_Packed,
                    >,
                    u32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                4usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__PollNextEventPacked as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (pEvent, uncbVREvent, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t_Packed>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__PollNextEventPacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VREvent_t_Packed,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                ),
                bool,
                2usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__PollNextEventPacked as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pEvent, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t_Packed>,
        uncbVREvent: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__PollNextEventPacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::VREvent_t_Packed,
                    >,
                    u32,
                ),
                bool,
                2usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__PollNextEventPacked as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pEvent, uncbVREvent))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRSystem__PollNextEventPacked as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRSystem__PollNextEventPacked as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+_PollNextEventPacked")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::CVRSystem__PollNextEventPacked {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
