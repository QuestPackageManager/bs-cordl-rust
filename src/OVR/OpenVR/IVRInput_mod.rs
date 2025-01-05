#[cfg(feature = "OVR+OpenVR+IVRInput")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IVRInput {
    pub SetActionManifestPath: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__SetActionManifestPath,
    >,
    pub GetActionSetHandle: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetActionSetHandle,
    >,
    pub GetActionHandle: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetActionHandle,
    >,
    pub GetInputSourceHandle: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetInputSourceHandle,
    >,
    pub UpdateActionState: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__UpdateActionState,
    >,
    pub GetDigitalActionData: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetDigitalActionData,
    >,
    pub GetAnalogActionData: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetAnalogActionData,
    >,
    pub GetPoseActionData: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetPoseActionData,
    >,
    pub GetSkeletalActionData: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetSkeletalActionData,
    >,
    pub GetSkeletalBoneData: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetSkeletalBoneData,
    >,
    pub GetSkeletalBoneDataCompressed: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetSkeletalBoneDataCompressed,
    >,
    pub DecompressSkeletalBoneData: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__DecompressSkeletalBoneData,
    >,
    pub TriggerHapticVibrationAction: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__TriggerHapticVibrationAction,
    >,
    pub GetActionOrigins: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetActionOrigins,
    >,
    pub GetOriginLocalizedName: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetOriginLocalizedName,
    >,
    pub GetOriginTrackedDeviceInfo: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__GetOriginTrackedDeviceInfo,
    >,
    pub ShowActionOrigins: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__ShowActionOrigins,
    >,
    pub ShowBindingsForActionSet: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRInput__ShowBindingsForActionSet,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRInput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput => "OVR.OpenVR"."IVRInput"
);
#[cfg(feature = "OVR+OpenVR+IVRInput")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVRInput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput")]
impl crate::OVR::OpenVR::IVRInput {
    #[cfg(feature = "OVR+OpenVR+IVRInput+_DecompressSkeletalBoneData")]
    pub type _DecompressSkeletalBoneData = crate::OVR::OpenVR::IVRInput__DecompressSkeletalBoneData;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionHandle")]
    pub type _GetActionHandle = crate::OVR::OpenVR::IVRInput__GetActionHandle;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionOrigins")]
    pub type _GetActionOrigins = crate::OVR::OpenVR::IVRInput__GetActionOrigins;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionSetHandle")]
    pub type _GetActionSetHandle = crate::OVR::OpenVR::IVRInput__GetActionSetHandle;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetAnalogActionData")]
    pub type _GetAnalogActionData = crate::OVR::OpenVR::IVRInput__GetAnalogActionData;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetDigitalActionData")]
    pub type _GetDigitalActionData = crate::OVR::OpenVR::IVRInput__GetDigitalActionData;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetInputSourceHandle")]
    pub type _GetInputSourceHandle = crate::OVR::OpenVR::IVRInput__GetInputSourceHandle;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginLocalizedName")]
    pub type _GetOriginLocalizedName = crate::OVR::OpenVR::IVRInput__GetOriginLocalizedName;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginTrackedDeviceInfo")]
    pub type _GetOriginTrackedDeviceInfo = crate::OVR::OpenVR::IVRInput__GetOriginTrackedDeviceInfo;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetPoseActionData")]
    pub type _GetPoseActionData = crate::OVR::OpenVR::IVRInput__GetPoseActionData;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalActionData")]
    pub type _GetSkeletalActionData = crate::OVR::OpenVR::IVRInput__GetSkeletalActionData;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneData")]
    pub type _GetSkeletalBoneData = crate::OVR::OpenVR::IVRInput__GetSkeletalBoneData;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneDataCompressed")]
    pub type _GetSkeletalBoneDataCompressed = crate::OVR::OpenVR::IVRInput__GetSkeletalBoneDataCompressed;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_SetActionManifestPath")]
    pub type _SetActionManifestPath = crate::OVR::OpenVR::IVRInput__SetActionManifestPath;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_ShowActionOrigins")]
    pub type _ShowActionOrigins = crate::OVR::OpenVR::IVRInput__ShowActionOrigins;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_ShowBindingsForActionSet")]
    pub type _ShowBindingsForActionSet = crate::OVR::OpenVR::IVRInput__ShowBindingsForActionSet;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_TriggerHapticVibrationAction")]
    pub type _TriggerHapticVibrationAction = crate::OVR::OpenVR::IVRInput__TriggerHapticVibrationAction;
    #[cfg(feature = "OVR+OpenVR+IVRInput+_UpdateActionState")]
    pub type _UpdateActionState = crate::OVR::OpenVR::IVRInput__UpdateActionState;
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_DecompressSkeletalBoneData")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__DecompressSkeletalBoneData {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_DecompressSkeletalBoneData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRInput__DecompressSkeletalBoneData => "OVR.OpenVR"
    ."IVRInput/_DecompressSkeletalBoneData"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_DecompressSkeletalBoneData")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__DecompressSkeletalBoneData {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_DecompressSkeletalBoneData")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__DecompressSkeletalBoneData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_DecompressSkeletalBoneData")]
impl crate::OVR::OpenVR::IVRInput__DecompressSkeletalBoneData {
    pub fn BeginInvoke(
        &mut self,
        pvCompressedBuffer: crate::System::IntPtr,
        unCompressedBufferSize: u32,
        peTransformSpace: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        >,
        pTransformArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::VRBoneTransform_t>,
            >,
        >,
        unTransformArrayCount: u32,
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
                    pvCompressedBuffer,
                    unCompressedBufferSize,
                    peTransformSpace,
                    pTransformArray,
                    unTransformArrayCount,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peTransformSpace: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (peTransformSpace, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pvCompressedBuffer: crate::System::IntPtr,
        unCompressedBufferSize: u32,
        peTransformSpace: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        >,
        pTransformArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::VRBoneTransform_t>,
            >,
        >,
        unTransformArrayCount: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "Invoke",
                (
                    pvCompressedBuffer,
                    unCompressedBufferSize,
                    peTransformSpace,
                    pTransformArray,
                    unTransformArrayCount,
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_DecompressSkeletalBoneData")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__DecompressSkeletalBoneData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetActionHandle {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__GetActionHandle =>
    "OVR.OpenVR"."IVRInput/_GetActionHandle"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionHandle")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetActionHandle {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionHandle")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetActionHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionHandle")]
impl crate::OVR::OpenVR::IVRInput__GetActionHandle {
    pub fn BeginInvoke(
        &mut self,
        pchActionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchActionName, pHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (pHandle, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchActionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("Invoke", (pchActionName, pHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetActionHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionOrigins")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetActionOrigins {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionOrigins")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__GetActionOrigins =>
    "OVR.OpenVR"."IVRInput/_GetActionOrigins"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionOrigins")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetActionOrigins {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionOrigins")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetActionOrigins {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionOrigins")]
impl crate::OVR::OpenVR::IVRInput__GetActionOrigins {
    pub fn BeginInvoke(
        &mut self,
        actionSetHandle: u64,
        digitalActionHandle: u64,
        originsOut: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        >,
        originOutCount: u32,
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
                    actionSetHandle,
                    digitalActionHandle,
                    originsOut,
                    originOutCount,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        actionSetHandle: u64,
        digitalActionHandle: u64,
        originsOut: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        >,
        originOutCount: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "Invoke",
                (actionSetHandle, digitalActionHandle, originsOut, originOutCount),
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionOrigins")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetActionOrigins {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionSetHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetActionSetHandle {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionSetHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__GetActionSetHandle =>
    "OVR.OpenVR"."IVRInput/_GetActionSetHandle"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionSetHandle")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetActionSetHandle {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionSetHandle")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetActionSetHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionSetHandle")]
impl crate::OVR::OpenVR::IVRInput__GetActionSetHandle {
    pub fn BeginInvoke(
        &mut self,
        pchActionSetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchActionSetName, pHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (pHandle, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchActionSetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("Invoke", (pchActionSetName, pHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetActionSetHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetActionSetHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetAnalogActionData")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetAnalogActionData {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetAnalogActionData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__GetAnalogActionData =>
    "OVR.OpenVR"."IVRInput/_GetAnalogActionData"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetAnalogActionData")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetAnalogActionData {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetAnalogActionData")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetAnalogActionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetAnalogActionData")]
impl crate::OVR::OpenVR::IVRInput__GetAnalogActionData {
    pub fn BeginInvoke(
        &mut self,
        action: u64,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputAnalogActionData_t,
        >,
        unActionDataSize: u32,
        ulRestrictToDevice: u64,
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
                    action,
                    pActionData,
                    unActionDataSize,
                    ulRestrictToDevice,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputAnalogActionData_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (pActionData, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
                "Invoke",
                (action, pActionData, unActionDataSize, ulRestrictToDevice),
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetAnalogActionData")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetAnalogActionData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetDigitalActionData")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetDigitalActionData {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetDigitalActionData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__GetDigitalActionData =>
    "OVR.OpenVR"."IVRInput/_GetDigitalActionData"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetDigitalActionData")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetDigitalActionData {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetDigitalActionData")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetDigitalActionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetDigitalActionData")]
impl crate::OVR::OpenVR::IVRInput__GetDigitalActionData {
    pub fn BeginInvoke(
        &mut self,
        action: u64,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputDigitalActionData_t,
        >,
        unActionDataSize: u32,
        ulRestrictToDevice: u64,
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
                    action,
                    pActionData,
                    unActionDataSize,
                    ulRestrictToDevice,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputDigitalActionData_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (pActionData, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
                "Invoke",
                (action, pActionData, unActionDataSize, ulRestrictToDevice),
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetDigitalActionData")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetDigitalActionData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetInputSourceHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetInputSourceHandle {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetInputSourceHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__GetInputSourceHandle =>
    "OVR.OpenVR"."IVRInput/_GetInputSourceHandle"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetInputSourceHandle")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetInputSourceHandle {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetInputSourceHandle")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetInputSourceHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetInputSourceHandle")]
impl crate::OVR::OpenVR::IVRInput__GetInputSourceHandle {
    pub fn BeginInvoke(
        &mut self,
        pchInputSourcePath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchInputSourcePath, pHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (pHandle, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchInputSourcePath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("Invoke", (pchInputSourcePath, pHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetInputSourceHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetInputSourceHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginLocalizedName")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetOriginLocalizedName {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginLocalizedName")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__GetOriginLocalizedName =>
    "OVR.OpenVR"."IVRInput/_GetOriginLocalizedName"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginLocalizedName")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetOriginLocalizedName {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginLocalizedName")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetOriginLocalizedName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginLocalizedName")]
impl crate::OVR::OpenVR::IVRInput__GetOriginLocalizedName {
    pub fn BeginInvoke(
        &mut self,
        origin: u64,
        pchNameArray: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unNameArraySize: u32,
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
                (origin, pchNameArray, unNameArraySize, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        origin: u64,
        pchNameArray: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unNameArraySize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("Invoke", (origin, pchNameArray, unNameArraySize))?;
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginLocalizedName")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetOriginLocalizedName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginTrackedDeviceInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetOriginTrackedDeviceInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginTrackedDeviceInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRInput__GetOriginTrackedDeviceInfo => "OVR.OpenVR"
    ."IVRInput/_GetOriginTrackedDeviceInfo"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginTrackedDeviceInfo")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetOriginTrackedDeviceInfo {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginTrackedDeviceInfo")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetOriginTrackedDeviceInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginTrackedDeviceInfo")]
impl crate::OVR::OpenVR::IVRInput__GetOriginTrackedDeviceInfo {
    pub fn BeginInvoke(
        &mut self,
        origin: u64,
        pOriginInfo: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputOriginInfo_t,
        >,
        unOriginInfoSize: u32,
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
                (origin, pOriginInfo, unOriginInfoSize, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pOriginInfo: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputOriginInfo_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (pOriginInfo, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
            .invoke("Invoke", (origin, pOriginInfo, unOriginInfoSize))?;
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetOriginTrackedDeviceInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetOriginTrackedDeviceInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetPoseActionData")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetPoseActionData {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetPoseActionData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__GetPoseActionData =>
    "OVR.OpenVR"."IVRInput/_GetPoseActionData"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetPoseActionData")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetPoseActionData {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetPoseActionData")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetPoseActionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetPoseActionData")]
impl crate::OVR::OpenVR::IVRInput__GetPoseActionData {
    pub fn BeginInvoke(
        &mut self,
        action: u64,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        fPredictedSecondsFromNow: f32,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputPoseActionData_t,
        >,
        unActionDataSize: u32,
        ulRestrictToDevice: u64,
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
                    action,
                    eOrigin,
                    fPredictedSecondsFromNow,
                    pActionData,
                    unActionDataSize,
                    ulRestrictToDevice,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputPoseActionData_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (pActionData, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
                "Invoke",
                (
                    action,
                    eOrigin,
                    fPredictedSecondsFromNow,
                    pActionData,
                    unActionDataSize,
                    ulRestrictToDevice,
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetPoseActionData")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetPoseActionData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalActionData")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetSkeletalActionData {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalActionData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__GetSkeletalActionData =>
    "OVR.OpenVR"."IVRInput/_GetSkeletalActionData"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalActionData")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetSkeletalActionData {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalActionData")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetSkeletalActionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalActionData")]
impl crate::OVR::OpenVR::IVRInput__GetSkeletalActionData {
    pub fn BeginInvoke(
        &mut self,
        action: u64,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputSkeletalActionData_t,
        >,
        unActionDataSize: u32,
        ulRestrictToDevice: u64,
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
                    action,
                    pActionData,
                    unActionDataSize,
                    ulRestrictToDevice,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pActionData: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::InputSkeletalActionData_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (pActionData, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
                "Invoke",
                (action, pActionData, unActionDataSize, ulRestrictToDevice),
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalActionData")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetSkeletalActionData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneData")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetSkeletalBoneData {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__GetSkeletalBoneData =>
    "OVR.OpenVR"."IVRInput/_GetSkeletalBoneData"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneData")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetSkeletalBoneData {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneData")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetSkeletalBoneData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneData")]
impl crate::OVR::OpenVR::IVRInput__GetSkeletalBoneData {
    pub fn BeginInvoke(
        &mut self,
        action: u64,
        eTransformSpace: crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        eMotionRange: crate::OVR::OpenVR::EVRSkeletalMotionRange,
        pTransformArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::VRBoneTransform_t>,
            >,
        >,
        unTransformArrayCount: u32,
        ulRestrictToDevice: u64,
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
                    action,
                    eTransformSpace,
                    eMotionRange,
                    pTransformArray,
                    unTransformArrayCount,
                    ulRestrictToDevice,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        action: u64,
        eTransformSpace: crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        eMotionRange: crate::OVR::OpenVR::EVRSkeletalMotionRange,
        pTransformArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::VRBoneTransform_t>,
            >,
        >,
        unTransformArrayCount: u32,
        ulRestrictToDevice: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "Invoke",
                (
                    action,
                    eTransformSpace,
                    eMotionRange,
                    pTransformArray,
                    unTransformArrayCount,
                    ulRestrictToDevice,
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneData")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetSkeletalBoneData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneDataCompressed")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__GetSkeletalBoneDataCompressed {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneDataCompressed")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRInput__GetSkeletalBoneDataCompressed => "OVR.OpenVR"
    ."IVRInput/_GetSkeletalBoneDataCompressed"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneDataCompressed")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__GetSkeletalBoneDataCompressed {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneDataCompressed")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__GetSkeletalBoneDataCompressed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneDataCompressed")]
impl crate::OVR::OpenVR::IVRInput__GetSkeletalBoneDataCompressed {
    pub fn BeginInvoke(
        &mut self,
        action: u64,
        eTransformSpace: crate::OVR::OpenVR::EVRSkeletalTransformSpace,
        eMotionRange: crate::OVR::OpenVR::EVRSkeletalMotionRange,
        pvCompressedData: crate::System::IntPtr,
        unCompressedSize: u32,
        punRequiredCompressedSize: quest_hook::libil2cpp::ByRefMut<u32>,
        ulRestrictToDevice: u64,
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
                    action,
                    eTransformSpace,
                    eMotionRange,
                    pvCompressedData,
                    unCompressedSize,
                    punRequiredCompressedSize,
                    ulRestrictToDevice,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        punRequiredCompressedSize: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (punRequiredCompressedSize, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
                "Invoke",
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_GetSkeletalBoneDataCompressed")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__GetSkeletalBoneDataCompressed {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_SetActionManifestPath")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__SetActionManifestPath {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_SetActionManifestPath")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__SetActionManifestPath =>
    "OVR.OpenVR"."IVRInput/_SetActionManifestPath"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_SetActionManifestPath")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__SetActionManifestPath {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_SetActionManifestPath")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__SetActionManifestPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_SetActionManifestPath")]
impl crate::OVR::OpenVR::IVRInput__SetActionManifestPath {
    pub fn BeginInvoke(
        &mut self,
        pchActionManifestPath: quest_hook::libil2cpp::Gc<
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
            .invoke("BeginInvoke", (pchActionManifestPath, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchActionManifestPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("Invoke", (pchActionManifestPath))?;
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_SetActionManifestPath")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__SetActionManifestPath {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowActionOrigins")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__ShowActionOrigins {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowActionOrigins")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__ShowActionOrigins =>
    "OVR.OpenVR"."IVRInput/_ShowActionOrigins"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowActionOrigins")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__ShowActionOrigins {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowActionOrigins")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__ShowActionOrigins {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowActionOrigins")]
impl crate::OVR::OpenVR::IVRInput__ShowActionOrigins {
    pub fn BeginInvoke(
        &mut self,
        actionSetHandle: u64,
        ulActionHandle: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (actionSetHandle, ulActionHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        actionSetHandle: u64,
        ulActionHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("Invoke", (actionSetHandle, ulActionHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowActionOrigins")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__ShowActionOrigins {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowBindingsForActionSet")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__ShowBindingsForActionSet {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowBindingsForActionSet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__ShowBindingsForActionSet
    => "OVR.OpenVR"."IVRInput/_ShowBindingsForActionSet"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowBindingsForActionSet")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__ShowBindingsForActionSet {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowBindingsForActionSet")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__ShowBindingsForActionSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowBindingsForActionSet")]
impl crate::OVR::OpenVR::IVRInput__ShowBindingsForActionSet {
    pub fn BeginInvoke(
        &mut self,
        pSets: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::OVR::OpenVR::VRActiveActionSet_t,
                >,
            >,
        >,
        unSizeOfVRSelectedActionSet_t: u32,
        unSetCount: u32,
        originToHighlight: u64,
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
                    pSets,
                    unSizeOfVRSelectedActionSet_t,
                    unSetCount,
                    originToHighlight,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pSets: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::OVR::OpenVR::VRActiveActionSet_t,
                >,
            >,
        >,
        unSizeOfVRSelectedActionSet_t: u32,
        unSetCount: u32,
        originToHighlight: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke(
                "Invoke",
                (pSets, unSizeOfVRSelectedActionSet_t, unSetCount, originToHighlight),
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_ShowBindingsForActionSet")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__ShowBindingsForActionSet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_TriggerHapticVibrationAction")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__TriggerHapticVibrationAction {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_TriggerHapticVibrationAction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRInput__TriggerHapticVibrationAction => "OVR.OpenVR"
    ."IVRInput/_TriggerHapticVibrationAction"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_TriggerHapticVibrationAction")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__TriggerHapticVibrationAction {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_TriggerHapticVibrationAction")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__TriggerHapticVibrationAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_TriggerHapticVibrationAction")]
impl crate::OVR::OpenVR::IVRInput__TriggerHapticVibrationAction {
    pub fn BeginInvoke(
        &mut self,
        action: u64,
        fStartSecondsFromNow: f32,
        fDurationSeconds: f32,
        fFrequency: f32,
        fAmplitude: f32,
        ulRestrictToDevice: u64,
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
                    action,
                    fStartSecondsFromNow,
                    fDurationSeconds,
                    fFrequency,
                    fAmplitude,
                    ulRestrictToDevice,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
                "Invoke",
                (
                    action,
                    fStartSecondsFromNow,
                    fDurationSeconds,
                    fFrequency,
                    fAmplitude,
                    ulRestrictToDevice,
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_TriggerHapticVibrationAction")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__TriggerHapticVibrationAction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_UpdateActionState")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRInput__UpdateActionState {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_UpdateActionState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRInput__UpdateActionState =>
    "OVR.OpenVR"."IVRInput/_UpdateActionState"
);
#[cfg(feature = "OVR+OpenVR+IVRInput+_UpdateActionState")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRInput__UpdateActionState {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_UpdateActionState")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRInput__UpdateActionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRInput+_UpdateActionState")]
impl crate::OVR::OpenVR::IVRInput__UpdateActionState {
    pub fn BeginInvoke(
        &mut self,
        pSets: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::OVR::OpenVR::VRActiveActionSet_t,
                >,
            >,
        >,
        unSizeOfVRSelectedActionSet_t: u32,
        unSetCount: u32,
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
                (pSets, unSizeOfVRSelectedActionSet_t, unSetCount, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pSets: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::OVR::OpenVR::VRActiveActionSet_t,
                >,
            >,
        >,
        unSizeOfVRSelectedActionSet_t: u32,
        unSetCount: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRInputError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRInputError = __cordl_object
            .invoke("Invoke", (pSets, unSizeOfVRSelectedActionSet_t, unSetCount))?;
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
#[cfg(feature = "OVR+OpenVR+IVRInput+_UpdateActionState")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRInput__UpdateActionState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
