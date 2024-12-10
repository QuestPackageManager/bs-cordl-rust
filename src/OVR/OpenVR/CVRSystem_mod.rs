#[cfg(feature = "OVR+OpenVR+CVRSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRSystem,
}
#[cfg(feature = "OVR+OpenVR+CVRSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRSystem => "OVR.OpenVR"
    ."CVRSystem"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AcknowledgeQuit_Exiting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AcknowledgeQuit_UserPrompt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AcknowledgeQuit_UserPrompt", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyTransform", (pOutputPose, pTrackedDevicePose, pTransform))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ComputeDistortion", (eEye, fU, fV, pDistortionCoordinates))?;
        Ok(__cordl_ret.into())
    }
    pub fn DriverDebugRequest(
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
                "DriverDebugRequest",
                (unDeviceIndex, pchRequest, pchResponseBuffer, unResponseBufferSize),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetArrayTrackedDeviceProperty",
                (unDeviceIndex, prop, propType, pBuffer, unBufferSize, pError),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetBoolTrackedDeviceProperty", (unDeviceIndex, prop, pError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonIdNameFromEnum(
        &mut self,
        eButtonId: crate::OVR::OpenVR::EVRButtonId,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetButtonIdNameFromEnum", (eButtonId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerAxisTypeNameFromEnum(
        &mut self,
        eAxisType: crate::OVR::OpenVR::EVRControllerAxisType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetControllerAxisTypeNameFromEnum", (eAxisType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerRoleForTrackedDeviceIndex(
        &mut self,
        unDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackedControllerRole> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::ETrackedControllerRole = __cordl_object
            .invoke("GetControllerRoleForTrackedDeviceIndex", (unDeviceIndex))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetControllerState",
                (unControllerDeviceIndex, pControllerState, unControllerStateSize),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetControllerStateWithPose",
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
    pub fn GetD3D9AdapterIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetD3D9AdapterIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDXGIOutputInfo(
        &mut self,
        pnAdapterIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDXGIOutputInfo", (pnAdapterIndex))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetDeviceToAbsoluteTrackingPose",
                (eOrigin, fPredictedSecondsToPhotonsFromNow, pTrackedDevicePoseArray),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEventTypeNameFromEnum(
        &mut self,
        eType: crate::OVR::OpenVR::EVREventType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetEventTypeNameFromEnum", (eType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeToHeadTransform(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
            .invoke("GetEyeToHeadTransform", (eEye))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetFloatTrackedDeviceProperty", (unDeviceIndex, prop, pError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHiddenAreaMesh(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        _cordl_type: crate::OVR::OpenVR::EHiddenAreaMeshType,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HiddenAreaMesh_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HiddenAreaMesh_t = __cordl_object
            .invoke("GetHiddenAreaMesh", (eEye, _cordl_type))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetInt32TrackedDeviceProperty", (unDeviceIndex, prop, pError))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
            .invoke("GetMatrix34TrackedDeviceProperty", (unDeviceIndex, prop, pError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputDevice(
        &mut self,
        pnDevice: quest_hook::libil2cpp::ByRefMut<u64>,
        textureType: crate::OVR::OpenVR::ETextureType,
        pInstance: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOutputDevice", (pnDevice, textureType, pInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProjectionMatrix(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        fNearZ: f32,
        fFarZ: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix44_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix44_t = __cordl_object
            .invoke("GetProjectionMatrix", (eEye, fNearZ, fFarZ))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetProjectionRaw", (eEye, pfLeft, pfRight, pfTop, pfBottom))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropErrorNameFromEnum(
        &mut self,
        error: crate::OVR::OpenVR::ETrackedPropertyError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPropErrorNameFromEnum", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRawZeroPoseToStandingAbsoluteTrackingPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
            .invoke("GetRawZeroPoseToStandingAbsoluteTrackingPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRecommendedRenderTargetSize(
        &mut self,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetRecommendedRenderTargetSize", (pnWidth, pnHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSeatedZeroPoseToStandingAbsoluteTrackingPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = __cordl_object
            .invoke("GetSeatedZeroPoseToStandingAbsoluteTrackingPose", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetSortedTrackedDeviceIndicesOfClass",
                (
                    eTrackedDeviceClass,
                    punTrackedDeviceIndexArray,
                    unRelativeToTrackedDeviceIndex,
                ),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetStringTrackedDeviceProperty",
                (unDeviceIndex, prop, pchValue, unBufferSize, pError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeSinceLastVsync(
        &mut self,
        pfSecondsSinceLastVsync: quest_hook::libil2cpp::ByRefMut<f32>,
        pulFrameCounter: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetTimeSinceLastVsync",
                (pfSecondsSinceLastVsync, pulFrameCounter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackedDeviceActivityLevel(
        &mut self,
        unDeviceId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EDeviceActivityLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EDeviceActivityLevel = __cordl_object
            .invoke("GetTrackedDeviceActivityLevel", (unDeviceId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackedDeviceClass(
        &mut self,
        unDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::ETrackedDeviceClass> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::ETrackedDeviceClass = __cordl_object
            .invoke("GetTrackedDeviceClass", (unDeviceIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackedDeviceIndexForControllerRole(
        &mut self,
        unDeviceType: crate::OVR::OpenVR::ETrackedControllerRole,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetTrackedDeviceIndexForControllerRole", (unDeviceType))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object
            .invoke("GetUint64TrackedDeviceProperty", (unDeviceIndex, prop, pError))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDisplayOnDesktop(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDisplayOnDesktop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInputAvailable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInputAvailable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSteamVRDrawingControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSteamVRDrawingControllers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTrackedDeviceConnected(
        &mut self,
        unDeviceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsTrackedDeviceConnected", (unDeviceIndex))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRFirmwareError = __cordl_object
            .invoke("PerformFirmwareUpdate", (unDeviceIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn PollNextEvent(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PollNextEvent", (pEvent, uncbVREvent))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "PollNextEventWithPose",
                (eOrigin, pEvent, uncbVREvent, pTrackedDevicePose),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetSeatedZeroPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetSeatedZeroPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDisplayVisibility(
        &mut self,
        bIsVisibleOnDesktop: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetDisplayVisibility", (bIsVisibleOnDesktop))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldApplicationPause(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldApplicationPause", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldApplicationReduceRenderingWork(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldApplicationReduceRenderingWork", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerHapticPulse(
        &mut self,
        unControllerDeviceIndex: u32,
        unAxisId: u32,
        usDurationMicroSec: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TriggerHapticPulse",
                (unControllerDeviceIndex, unAxisId, usDurationMicroSec),
            )?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone)]
pub struct CVRSystem_GetControllerStateUnion {
    padding: [u8; 8usize],
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateUnion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRSystem_GetControllerStateUnion
    => "OVR.OpenVR"."CVRSystem/GetControllerStateUnion"
);
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
#[derive(Debug, Clone)]
pub struct CVRSystem_GetControllerStateWithPoseUnion {
    padding: [u8; 8usize],
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+GetControllerStateWithPoseUnion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::CVRSystem_GetControllerStateWithPoseUnion => "OVR.OpenVR"
    ."CVRSystem/GetControllerStateWithPoseUnion"
);
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
#[derive(Debug, Clone)]
pub struct CVRSystem_PollNextEventUnion {
    padding: [u8; 8usize],
}
#[cfg(feature = "OVR+OpenVR+CVRSystem+PollNextEventUnion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRSystem_PollNextEventUnion =>
    "OVR.OpenVR"."CVRSystem/PollNextEventUnion"
);
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRSystem__GetControllerStatePacked
    => "OVR.OpenVR"."CVRSystem/_GetControllerStatePacked"
);
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
            crate::OVR::OpenVR::VRControllerState_t_Packed,
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
            crate::OVR::OpenVR::VRControllerState_t_Packed,
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::CVRSystem__GetControllerStateWithPosePacked => "OVR.OpenVR"
    ."CVRSystem/_GetControllerStateWithPosePacked"
);
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
            crate::OVR::OpenVR::VRControllerState_t_Packed,
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
            crate::OVR::OpenVR::VRControllerState_t_Packed,
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRSystem__PollNextEventPacked =>
    "OVR.OpenVR"."CVRSystem/_PollNextEventPacked"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pEvent, uncbVREvent, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t_Packed>,
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
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t_Packed>,
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
