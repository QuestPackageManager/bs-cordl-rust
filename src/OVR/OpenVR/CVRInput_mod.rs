#[cfg(feature = "OVR+OpenVR+CVRInput")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRInput {
    __cordl_parent: crate::System::Object,
    pub FnTable: crate::OVR::OpenVR::IVRInput,
}
#[cfg(feature = "OVR+OpenVR+CVRInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRInput => "OVR.OpenVR"."CVRInput"
);
#[cfg(feature = "OVR+OpenVR+CVRInput")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRInput {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRInput")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRInput")]
impl crate::OVR::OpenVR::CVRInput {
    pub fn GetActionHandle(
        &mut self,
        pchActionName: *mut crate::System::String,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("GetActionHandle", (pchActionName, pHandle))?;
        Ok(__cordl_ret)
    }
    pub fn GetInputSourceHandle(
        &mut self,
        pchInputSourcePath: *mut crate::System::String,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("GetInputSourceHandle", (pchInputSourcePath, pHandle))?;
        Ok(__cordl_ret)
    }
    pub fn GetActionOrigins(
        &mut self,
        actionSetHandle: u64,
        digitalActionHandle: u64,
        originsOut: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "GetActionOrigins",
                (actionSetHandle, digitalActionHandle, originsOut),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateActionState(
        &mut self,
        pSets: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::VRActiveActionSet_t,
        >,
        unSizeOfVRSelectedActionSet_t: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("UpdateActionState", (pSets, unSizeOfVRSelectedActionSet_t))?;
        Ok(__cordl_ret)
    }
    pub fn GetOriginLocalizedName(
        &mut self,
        origin: u64,
        pchNameArray: *mut crate::System::Text::StringBuilder,
        unNameArraySize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("GetOriginLocalizedName", (origin, pchNameArray, unNameArraySize))?;
        Ok(__cordl_ret)
    }
    pub fn GetPoseActionData(
        &mut self,
        action: u64,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        fPredictedSecondsFromNow: f32,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputPoseActionData_t,
        >,
        unActionDataSize: u32,
        ulRestrictToDevice: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "GetPoseActionData",
                (
                    action,
                    eOrigin,
                    fPredictedSecondsFromNow,
                    pActionData,
                    unActionDataSize,
                    ulRestrictToDevice,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetSkeletalBoneData(
        &mut self,
        action: u64,
        eTransformSpace: crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        eMotionRange: crate::OVR::OpenVR::EVRSkeletalMotionRange,
        pTransformArray: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::VRBoneTransform_t,
        >,
        ulRestrictToDevice: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "GetSkeletalBoneData",
                (
                    action,
                    eTransformSpace,
                    eMotionRange,
                    pTransformArray,
                    ulRestrictToDevice,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetSkeletalBoneDataCompressed(
        &mut self,
        action: u64,
        eTransformSpace: crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        eMotionRange: crate::OVR::OpenVR::EVRSkeletalMotionRange,
        pvCompressedData: crate::System::IntPtr,
        unCompressedSize: u32,
        punRequiredCompressedSize: quest_hook::libil2cpp::ByRefMut<u32>,
        ulRestrictToDevice: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "GetSkeletalBoneDataCompressed",
                (
                    action,
                    eTransformSpace,
                    eMotionRange,
                    pvCompressedData,
                    unCompressedSize,
                    punRequiredCompressedSize,
                    ulRestrictToDevice,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetDigitalActionData(
        &mut self,
        action: u64,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputDigitalActionData_t,
        >,
        unActionDataSize: u32,
        ulRestrictToDevice: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "GetDigitalActionData",
                (action, pActionData, unActionDataSize, ulRestrictToDevice),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ShowActionOrigins(
        &mut self,
        actionSetHandle: u64,
        ulActionHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("ShowActionOrigins", (actionSetHandle, ulActionHandle))?;
        Ok(__cordl_ret)
    }
    pub fn GetActionSetHandle(
        &mut self,
        pchActionSetName: *mut crate::System::String,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("GetActionSetHandle", (pchActionSetName, pHandle))?;
        Ok(__cordl_ret)
    }
    pub fn ShowBindingsForActionSet(
        &mut self,
        pSets: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::VRActiveActionSet_t,
        >,
        unSizeOfVRSelectedActionSet_t: u32,
        originToHighlight: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "ShowBindingsForActionSet",
                (pSets, unSizeOfVRSelectedActionSet_t, originToHighlight),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetAnalogActionData(
        &mut self,
        action: u64,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputAnalogActionData_t,
        >,
        unActionDataSize: u32,
        ulRestrictToDevice: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "GetAnalogActionData",
                (action, pActionData, unActionDataSize, ulRestrictToDevice),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetActionManifestPath(
        &mut self,
        pchActionManifestPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("SetActionManifestPath", (pchActionManifestPath))?;
        Ok(__cordl_ret)
    }
    pub fn TriggerHapticVibrationAction(
        &mut self,
        action: u64,
        fStartSecondsFromNow: f32,
        fDurationSeconds: f32,
        fFrequency: f32,
        fAmplitude: f32,
        ulRestrictToDevice: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "TriggerHapticVibrationAction",
                (
                    action,
                    fStartSecondsFromNow,
                    fDurationSeconds,
                    fFrequency,
                    fAmplitude,
                    ulRestrictToDevice,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOriginTrackedDeviceInfo(
        &mut self,
        origin: u64,
        pOriginInfo: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputOriginInfo_t,
        >,
        unOriginInfoSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "GetOriginTrackedDeviceInfo",
                (origin, pOriginInfo, unOriginInfoSize),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DecompressSkeletalBoneData(
        &mut self,
        pvCompressedBuffer: crate::System::IntPtr,
        unCompressedBufferSize: u32,
        peTransformSpace: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        >,
        pTransformArray: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::VRBoneTransform_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "DecompressSkeletalBoneData",
                (
                    pvCompressedBuffer,
                    unCompressedBufferSize,
                    peTransformSpace,
                    pTransformArray,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetSkeletalActionData(
        &mut self,
        action: u64,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputSkeletalActionData_t,
        >,
        unActionDataSize: u32,
        ulRestrictToDevice: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "GetSkeletalActionData",
                (action, pActionData, unActionDataSize, ulRestrictToDevice),
            )?;
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
#[cfg(feature = "OVR+OpenVR+CVRInput")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
