#[cfg(feature = "OVR+OpenVR+CVRInput")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRInput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRInput,
}
#[cfg(feature = "OVR+OpenVR+CVRInput")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRInput {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRInput";
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
#[cfg(feature = "OVR+OpenVR+CVRInput")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRInput {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRInput")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRInput {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRInput")]
impl crate::OVR::OpenVR::CVRInput {
    pub fn DecompressSkeletalBoneData(
        &mut self,
        pvCompressedBuffer: crate::System::IntPtr,
        unCompressedBufferSize: u32,
        peTransformSpace: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        >,
        pTransformArray: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::VRBoneTransform_t>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::IntPtr,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::EVRSkeletalTransformSpace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::OVR::OpenVR::VRBoneTransform_t,
                                >,
                            >,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        4usize,
                    >("DecompressSkeletalBoneData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DecompressSkeletalBoneData", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        pvCompressedBuffer,
                        unCompressedBufferSize,
                        peTransformSpace,
                        pTransformArray,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionHandle(
        &mut self,
        pchActionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u64>,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        2usize,
                    >("GetActionHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetActionHandle", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info.invoke_unchecked(self, (pchActionName, pHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionOrigins(
        &mut self,
        actionSetHandle: u64,
        digitalActionHandle: u64,
        originsOut: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u64>,
                            >,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        3usize,
                    >("GetActionOrigins")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetActionOrigins", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (actionSetHandle, digitalActionHandle, originsOut),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionSetHandle(
        &mut self,
        pchActionSetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u64>,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        2usize,
                    >("GetActionSetHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetActionSetHandle", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info.invoke_unchecked(self, (pchActionSetName, pHandle))?
        };
        Ok(__cordl_ret.into())
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::InputAnalogActionData_t,
                            >,
                            u32,
                            u64,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        4usize,
                    >("GetAnalogActionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAnalogActionData", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (action, pActionData, unActionDataSize, ulRestrictToDevice),
                )?
        };
        Ok(__cordl_ret.into())
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::InputDigitalActionData_t,
                            >,
                            u32,
                            u64,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        4usize,
                    >("GetDigitalActionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDigitalActionData", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (action, pActionData, unActionDataSize, ulRestrictToDevice),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInputSourceHandle(
        &mut self,
        pchInputSourcePath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u64>,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        2usize,
                    >("GetInputSourceHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInputSourceHandle", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info.invoke_unchecked(self, (pchInputSourcePath, pHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOriginLocalizedName(
        &mut self,
        origin: u64,
        pchNameArray: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unNameArraySize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            u32,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        3usize,
                    >("GetOriginLocalizedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOriginLocalizedName", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (origin, pchNameArray, unNameArraySize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOriginTrackedDeviceInfo(
        &mut self,
        origin: u64,
        pOriginInfo: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputOriginInfo_t,
        >,
        unOriginInfoSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::InputOriginInfo_t,
                            >,
                            u32,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        3usize,
                    >("GetOriginTrackedDeviceInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOriginTrackedDeviceInfo", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (origin, pOriginInfo, unOriginInfoSize))?
        };
        Ok(__cordl_ret.into())
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::ETrackingUniverseOrigin,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::InputPoseActionData_t,
                            >,
                            u32,
                            u64,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        6usize,
                    >("GetPoseActionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPoseActionData", 6usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        action,
                        eOrigin,
                        fPredictedSecondsFromNow,
                        pActionData,
                        unActionDataSize,
                        ulRestrictToDevice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::InputSkeletalActionData_t,
                            >,
                            u32,
                            u64,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        4usize,
                    >("GetSkeletalActionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSkeletalActionData", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (action, pActionData, unActionDataSize, ulRestrictToDevice),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSkeletalBoneData(
        &mut self,
        action: u64,
        eTransformSpace: crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        eMotionRange: crate::OVR::OpenVR::EVRSkeletalMotionRange,
        pTransformArray: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::VRBoneTransform_t>,
        >,
        ulRestrictToDevice: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::EVRSkeletalTransformSpace,
                            crate::OVR::OpenVR::EVRSkeletalMotionRange,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::OVR::OpenVR::VRBoneTransform_t,
                                >,
                            >,
                            u64,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        5usize,
                    >("GetSkeletalBoneData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSkeletalBoneData", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        action,
                        eTransformSpace,
                        eMotionRange,
                        pTransformArray,
                        ulRestrictToDevice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::EVRSkeletalTransformSpace,
                            crate::OVR::OpenVR::EVRSkeletalMotionRange,
                            crate::System::IntPtr,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            u64,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        7usize,
                    >("GetSkeletalBoneDataCompressed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSkeletalBoneDataCompressed", 7usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        action,
                        eTransformSpace,
                        eMotionRange,
                        pvCompressedData,
                        unCompressedSize,
                        punRequiredCompressedSize,
                        ulRestrictToDevice,
                    ),
                )?
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
    pub fn SetActionManifestPath(
        &mut self,
        pchActionManifestPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::OVR::OpenVR::EVRInputError,
                        1usize,
                    >("SetActionManifestPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetActionManifestPath", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info.invoke_unchecked(self, (pchActionManifestPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowActionOrigins(
        &mut self,
        actionSetHandle: u64,
        ulActionHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, u64),
                        crate::OVR::OpenVR::EVRInputError,
                        2usize,
                    >("ShowActionOrigins")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShowActionOrigins", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info.invoke_unchecked(self, (actionSetHandle, ulActionHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowBindingsForActionSet(
        &mut self,
        pSets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::VRActiveActionSet_t>,
        >,
        unSizeOfVRSelectedActionSet_t: u32,
        originToHighlight: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::OVR::OpenVR::VRActiveActionSet_t,
                                >,
                            >,
                            u32,
                            u64,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        3usize,
                    >("ShowBindingsForActionSet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShowBindingsForActionSet", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (pSets, unSizeOfVRSelectedActionSet_t, originToHighlight),
                )?
        };
        Ok(__cordl_ret.into())
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, f32, f32, f32, f32, u64),
                        crate::OVR::OpenVR::EVRInputError,
                        6usize,
                    >("TriggerHapticVibrationAction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TriggerHapticVibrationAction", 6usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        action,
                        fStartSecondsFromNow,
                        fDurationSeconds,
                        fFrequency,
                        fAmplitude,
                        ulRestrictToDevice,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateActionState(
        &mut self,
        pSets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::VRActiveActionSet_t>,
        >,
        unSizeOfVRSelectedActionSet_t: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::OVR::OpenVR::VRActiveActionSet_t,
                                >,
                            >,
                            u32,
                        ),
                        crate::OVR::OpenVR::EVRInputError,
                        2usize,
                    >("UpdateActionState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateActionState", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (pSets, unSizeOfVRSelectedActionSet_t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (pInterface))?
        };
        Ok(__cordl_ret.into())
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
