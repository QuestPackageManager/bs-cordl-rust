#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IVRChaperoneSetup {
    pub CommitWorkingCopy: *mut crate::OVR::OpenVR::IVRChaperoneSetup__CommitWorkingCopy,
    pub RevertWorkingCopy: *mut crate::OVR::OpenVR::IVRChaperoneSetup__RevertWorkingCopy,
    pub GetWorkingPlayAreaSize: *mut crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaSize,
    pub GetWorkingPlayAreaRect: *mut crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaRect,
    pub GetWorkingCollisionBoundsInfo: *mut crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingCollisionBoundsInfo,
    pub GetLiveCollisionBoundsInfo: *mut crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsInfo,
    pub GetWorkingSeatedZeroPoseToRawTrackingPose: *mut crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingSeatedZeroPoseToRawTrackingPose,
    pub GetWorkingStandingZeroPoseToRawTrackingPose: *mut crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingStandingZeroPoseToRawTrackingPose,
    pub SetWorkingPlayAreaSize: *mut crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPlayAreaSize,
    pub SetWorkingCollisionBoundsInfo: *mut crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsInfo,
    pub SetWorkingSeatedZeroPoseToRawTrackingPose: *mut crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingSeatedZeroPoseToRawTrackingPose,
    pub SetWorkingStandingZeroPoseToRawTrackingPose: *mut crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingStandingZeroPoseToRawTrackingPose,
    pub ReloadFromDisk: *mut crate::OVR::OpenVR::IVRChaperoneSetup__ReloadFromDisk,
    pub GetLiveSeatedZeroPoseToRawTrackingPose: *mut crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveSeatedZeroPoseToRawTrackingPose,
    pub SetWorkingCollisionBoundsTagsInfo: *mut crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsTagsInfo,
    pub GetLiveCollisionBoundsTagsInfo: *mut crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsTagsInfo,
    pub SetWorkingPhysicalBoundsInfo: *mut crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPhysicalBoundsInfo,
    pub GetLivePhysicalBoundsInfo: *mut crate::OVR::OpenVR::IVRChaperoneSetup__GetLivePhysicalBoundsInfo,
    pub ExportLiveToBuffer: *mut crate::OVR::OpenVR::IVRChaperoneSetup__ExportLiveToBuffer,
    pub ImportFromBufferToWorking: *mut crate::OVR::OpenVR::IVRChaperoneSetup__ImportFromBufferToWorking,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRChaperoneSetup => "OVR.OpenVR"
    ."IVRChaperoneSetup"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::IVRChaperoneSetup {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup")]
impl crate::OVR::OpenVR::IVRChaperoneSetup {
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_CommitWorkingCopy")]
    pub type _CommitWorkingCopy = crate::OVR::OpenVR::IVRChaperoneSetup__CommitWorkingCopy;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ExportLiveToBuffer")]
    pub type _ExportLiveToBuffer = crate::OVR::OpenVR::IVRChaperoneSetup__ExportLiveToBuffer;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsInfo")]
    pub type _GetLiveCollisionBoundsInfo = crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsInfo;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsTagsInfo")]
    pub type _GetLiveCollisionBoundsTagsInfo = crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsTagsInfo;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLivePhysicalBoundsInfo")]
    pub type _GetLivePhysicalBoundsInfo = crate::OVR::OpenVR::IVRChaperoneSetup__GetLivePhysicalBoundsInfo;
    #[cfg(
        feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveSeatedZeroPoseToRawTrackingPose"
    )]
    pub type _GetLiveSeatedZeroPoseToRawTrackingPose = crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveSeatedZeroPoseToRawTrackingPose;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingCollisionBoundsInfo")]
    pub type _GetWorkingCollisionBoundsInfo = crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingCollisionBoundsInfo;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaRect")]
    pub type _GetWorkingPlayAreaRect = crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaRect;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaSize")]
    pub type _GetWorkingPlayAreaSize = crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaSize;
    #[cfg(
        feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingSeatedZeroPoseToRawTrackingPose"
    )]
    pub type _GetWorkingSeatedZeroPoseToRawTrackingPose = crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingSeatedZeroPoseToRawTrackingPose;
    #[cfg(
        feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingStandingZeroPoseToRawTrackingPose"
    )]
    pub type _GetWorkingStandingZeroPoseToRawTrackingPose = crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingStandingZeroPoseToRawTrackingPose;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ImportFromBufferToWorking")]
    pub type _ImportFromBufferToWorking = crate::OVR::OpenVR::IVRChaperoneSetup__ImportFromBufferToWorking;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ReloadFromDisk")]
    pub type _ReloadFromDisk = crate::OVR::OpenVR::IVRChaperoneSetup__ReloadFromDisk;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_RevertWorkingCopy")]
    pub type _RevertWorkingCopy = crate::OVR::OpenVR::IVRChaperoneSetup__RevertWorkingCopy;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsInfo")]
    pub type _SetWorkingCollisionBoundsInfo = crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsInfo;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsTagsInfo")]
    pub type _SetWorkingCollisionBoundsTagsInfo = crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsTagsInfo;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPhysicalBoundsInfo")]
    pub type _SetWorkingPhysicalBoundsInfo = crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPhysicalBoundsInfo;
    #[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPlayAreaSize")]
    pub type _SetWorkingPlayAreaSize = crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPlayAreaSize;
    #[cfg(
        feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingSeatedZeroPoseToRawTrackingPose"
    )]
    pub type _SetWorkingSeatedZeroPoseToRawTrackingPose = crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingSeatedZeroPoseToRawTrackingPose;
    #[cfg(
        feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingStandingZeroPoseToRawTrackingPose"
    )]
    pub type _SetWorkingStandingZeroPoseToRawTrackingPose = crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingStandingZeroPoseToRawTrackingPose;
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_CommitWorkingCopy")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__CommitWorkingCopy {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_CommitWorkingCopy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__CommitWorkingCopy => "OVR.OpenVR"
    ."IVRChaperoneSetup/_CommitWorkingCopy"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_CommitWorkingCopy")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRChaperoneSetup__CommitWorkingCopy {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_CommitWorkingCopy")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRChaperoneSetup__CommitWorkingCopy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_CommitWorkingCopy")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__CommitWorkingCopy {
    pub fn BeginInvoke(
        &mut self,
        configFile: crate::OVR::OpenVR::EChaperoneConfigFile,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (configFile, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        configFile: crate::OVR::OpenVR::EChaperoneConfigFile,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (configFile))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_CommitWorkingCopy")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__CommitWorkingCopy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ExportLiveToBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__ExportLiveToBuffer {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ExportLiveToBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__ExportLiveToBuffer => "OVR.OpenVR"
    ."IVRChaperoneSetup/_ExportLiveToBuffer"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ExportLiveToBuffer")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRChaperoneSetup__ExportLiveToBuffer {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ExportLiveToBuffer")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRChaperoneSetup__ExportLiveToBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ExportLiveToBuffer")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__ExportLiveToBuffer {
    pub fn BeginInvoke(
        &mut self,
        pBuffer: *mut crate::System::Text::StringBuilder,
        pnBufferLength: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pBuffer, pnBufferLength, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pnBufferLength: quest_hook::libil2cpp::ByRefMut<u32>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pnBufferLength, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pBuffer: *mut crate::System::Text::StringBuilder,
        pnBufferLength: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pBuffer, pnBufferLength))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ExportLiveToBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__ExportLiveToBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__GetLiveCollisionBoundsInfo {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsInfo => "OVR.OpenVR"
    ."IVRChaperoneSetup/_GetLiveCollisionBoundsInfo"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsInfo")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsInfo {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsInfo")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsInfo")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsInfo {
    pub fn BeginInvoke(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
        punQuadsCount: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pQuadsBuffer, punQuadsCount, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        punQuadsCount: quest_hook::libil2cpp::ByRefMut<u32>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (punQuadsCount, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
        punQuadsCount: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pQuadsBuffer, punQuadsCount))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsTagsInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__GetLiveCollisionBoundsTagsInfo {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsTagsInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsTagsInfo => "OVR.OpenVR"
    ."IVRChaperoneSetup/_GetLiveCollisionBoundsTagsInfo"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsTagsInfo")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsTagsInfo {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsTagsInfo")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsTagsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsTagsInfo")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsTagsInfo {
    pub fn BeginInvoke(
        &mut self,
        pTagsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        punTagCount: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pTagsBuffer, punTagCount, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        punTagCount: quest_hook::libil2cpp::ByRefMut<u32>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (punTagCount, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pTagsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        punTagCount: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pTagsBuffer, punTagCount))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveCollisionBoundsTagsInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveCollisionBoundsTagsInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLivePhysicalBoundsInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__GetLivePhysicalBoundsInfo {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLivePhysicalBoundsInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__GetLivePhysicalBoundsInfo => "OVR.OpenVR"
    ."IVRChaperoneSetup/_GetLivePhysicalBoundsInfo"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLivePhysicalBoundsInfo")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLivePhysicalBoundsInfo {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLivePhysicalBoundsInfo")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLivePhysicalBoundsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLivePhysicalBoundsInfo")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__GetLivePhysicalBoundsInfo {
    pub fn BeginInvoke(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
        punQuadsCount: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pQuadsBuffer, punQuadsCount, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        punQuadsCount: quest_hook::libil2cpp::ByRefMut<u32>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (punQuadsCount, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
        punQuadsCount: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pQuadsBuffer, punQuadsCount))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLivePhysicalBoundsInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLivePhysicalBoundsInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveSeatedZeroPoseToRawTrackingPose")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__GetLiveSeatedZeroPoseToRawTrackingPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveSeatedZeroPoseToRawTrackingPose")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__GetLiveSeatedZeroPoseToRawTrackingPose =>
    "OVR.OpenVR"."IVRChaperoneSetup/_GetLiveSeatedZeroPoseToRawTrackingPose"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveSeatedZeroPoseToRawTrackingPose")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveSeatedZeroPoseToRawTrackingPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveSeatedZeroPoseToRawTrackingPose")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveSeatedZeroPoseToRawTrackingPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveSeatedZeroPoseToRawTrackingPose")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveSeatedZeroPoseToRawTrackingPose {
    pub fn BeginInvoke(
        &mut self,
        pmatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (pmatSeatedZeroPoseToRawTrackingPose, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pmatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pmatSeatedZeroPoseToRawTrackingPose, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pmatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pmatSeatedZeroPoseToRawTrackingPose))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetLiveSeatedZeroPoseToRawTrackingPose")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__GetLiveSeatedZeroPoseToRawTrackingPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingCollisionBoundsInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__GetWorkingCollisionBoundsInfo {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingCollisionBoundsInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__GetWorkingCollisionBoundsInfo => "OVR.OpenVR"
    ."IVRChaperoneSetup/_GetWorkingCollisionBoundsInfo"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingCollisionBoundsInfo")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingCollisionBoundsInfo {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingCollisionBoundsInfo")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingCollisionBoundsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingCollisionBoundsInfo")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingCollisionBoundsInfo {
    pub fn BeginInvoke(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
        punQuadsCount: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pQuadsBuffer, punQuadsCount, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        punQuadsCount: quest_hook::libil2cpp::ByRefMut<u32>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (punQuadsCount, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
        punQuadsCount: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pQuadsBuffer, punQuadsCount))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingCollisionBoundsInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingCollisionBoundsInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaRect")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__GetWorkingPlayAreaRect {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaRect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaRect => "OVR.OpenVR"
    ."IVRChaperoneSetup/_GetWorkingPlayAreaRect"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaRect")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaRect {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaRect")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaRect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaRect")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaRect {
    pub fn BeginInvoke(
        &mut self,
        rect: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdQuad_t>,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (rect, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        rect: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdQuad_t>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (rect, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        rect: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdQuad_t>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (rect))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaRect")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaRect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaSize")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__GetWorkingPlayAreaSize {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaSize")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaSize => "OVR.OpenVR"
    ."IVRChaperoneSetup/_GetWorkingPlayAreaSize"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaSize")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaSize {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaSize")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaSize")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaSize {
    pub fn BeginInvoke(
        &mut self,
        pSizeX: quest_hook::libil2cpp::ByRefMut<f32>,
        pSizeZ: quest_hook::libil2cpp::ByRefMut<f32>,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pSizeX, pSizeZ, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pSizeX: quest_hook::libil2cpp::ByRefMut<f32>,
        pSizeZ: quest_hook::libil2cpp::ByRefMut<f32>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pSizeX, pSizeZ, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pSizeX: quest_hook::libil2cpp::ByRefMut<f32>,
        pSizeZ: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (pSizeX, pSizeZ))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingPlayAreaSize")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingPlayAreaSize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingSeatedZeroPoseToRawTrackingPose"
)]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__GetWorkingSeatedZeroPoseToRawTrackingPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingSeatedZeroPoseToRawTrackingPose"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__GetWorkingSeatedZeroPoseToRawTrackingPose =>
    "OVR.OpenVR"."IVRChaperoneSetup/_GetWorkingSeatedZeroPoseToRawTrackingPose"
);
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingSeatedZeroPoseToRawTrackingPose"
)]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingSeatedZeroPoseToRawTrackingPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingSeatedZeroPoseToRawTrackingPose"
)]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingSeatedZeroPoseToRawTrackingPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingSeatedZeroPoseToRawTrackingPose"
)]
impl crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingSeatedZeroPoseToRawTrackingPose {
    pub fn BeginInvoke(
        &mut self,
        pmatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (pmatSeatedZeroPoseToRawTrackingPose, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pmatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pmatSeatedZeroPoseToRawTrackingPose, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pmatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pmatSeatedZeroPoseToRawTrackingPose))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingSeatedZeroPoseToRawTrackingPose"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingSeatedZeroPoseToRawTrackingPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingStandingZeroPoseToRawTrackingPose"
)]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__GetWorkingStandingZeroPoseToRawTrackingPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingStandingZeroPoseToRawTrackingPose"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__GetWorkingStandingZeroPoseToRawTrackingPose =>
    "OVR.OpenVR"."IVRChaperoneSetup/_GetWorkingStandingZeroPoseToRawTrackingPose"
);
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingStandingZeroPoseToRawTrackingPose"
)]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingStandingZeroPoseToRawTrackingPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingStandingZeroPoseToRawTrackingPose"
)]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingStandingZeroPoseToRawTrackingPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingStandingZeroPoseToRawTrackingPose"
)]
impl crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingStandingZeroPoseToRawTrackingPose {
    pub fn BeginInvoke(
        &mut self,
        pmatStandingZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (pmatStandingZeroPoseToRawTrackingPose, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pmatStandingZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pmatStandingZeroPoseToRawTrackingPose, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pmatStandingZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pmatStandingZeroPoseToRawTrackingPose))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_GetWorkingStandingZeroPoseToRawTrackingPose"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__GetWorkingStandingZeroPoseToRawTrackingPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ImportFromBufferToWorking")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__ImportFromBufferToWorking {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ImportFromBufferToWorking")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__ImportFromBufferToWorking => "OVR.OpenVR"
    ."IVRChaperoneSetup/_ImportFromBufferToWorking"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ImportFromBufferToWorking")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__ImportFromBufferToWorking {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ImportFromBufferToWorking")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__ImportFromBufferToWorking {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ImportFromBufferToWorking")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__ImportFromBufferToWorking {
    pub fn BeginInvoke(
        &mut self,
        pBuffer: *mut crate::System::String,
        nImportFlags: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pBuffer, nImportFlags, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pBuffer: *mut crate::System::String,
        nImportFlags: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pBuffer, nImportFlags))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ImportFromBufferToWorking")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__ImportFromBufferToWorking {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ReloadFromDisk")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__ReloadFromDisk {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ReloadFromDisk")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRChaperoneSetup__ReloadFromDisk
    => "OVR.OpenVR"."IVRChaperoneSetup/_ReloadFromDisk"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ReloadFromDisk")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRChaperoneSetup__ReloadFromDisk {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ReloadFromDisk")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRChaperoneSetup__ReloadFromDisk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ReloadFromDisk")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__ReloadFromDisk {
    pub fn BeginInvoke(
        &mut self,
        configFile: crate::OVR::OpenVR::EChaperoneConfigFile,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (configFile, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        configFile: crate::OVR::OpenVR::EChaperoneConfigFile,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (configFile))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_ReloadFromDisk")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__ReloadFromDisk {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_RevertWorkingCopy")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__RevertWorkingCopy {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_RevertWorkingCopy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__RevertWorkingCopy => "OVR.OpenVR"
    ."IVRChaperoneSetup/_RevertWorkingCopy"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_RevertWorkingCopy")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRChaperoneSetup__RevertWorkingCopy {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_RevertWorkingCopy")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRChaperoneSetup__RevertWorkingCopy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_RevertWorkingCopy")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__RevertWorkingCopy {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_RevertWorkingCopy")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__RevertWorkingCopy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__SetWorkingCollisionBoundsInfo {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsInfo => "OVR.OpenVR"
    ."IVRChaperoneSetup/_SetWorkingCollisionBoundsInfo"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsInfo")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsInfo {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsInfo")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsInfo")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsInfo {
    pub fn BeginInvoke(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
        unQuadsCount: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pQuadsBuffer, unQuadsCount, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
        unQuadsCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pQuadsBuffer, unQuadsCount))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsTagsInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__SetWorkingCollisionBoundsTagsInfo {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsTagsInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsTagsInfo => "OVR.OpenVR"
    ."IVRChaperoneSetup/_SetWorkingCollisionBoundsTagsInfo"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsTagsInfo")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsTagsInfo {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsTagsInfo")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsTagsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsTagsInfo")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsTagsInfo {
    pub fn BeginInvoke(
        &mut self,
        pTagsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        unTagCount: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pTagsBuffer, unTagCount, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pTagsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        unTagCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pTagsBuffer, unTagCount))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingCollisionBoundsTagsInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingCollisionBoundsTagsInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPhysicalBoundsInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__SetWorkingPhysicalBoundsInfo {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPhysicalBoundsInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPhysicalBoundsInfo => "OVR.OpenVR"
    ."IVRChaperoneSetup/_SetWorkingPhysicalBoundsInfo"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPhysicalBoundsInfo")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPhysicalBoundsInfo {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPhysicalBoundsInfo")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPhysicalBoundsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPhysicalBoundsInfo")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPhysicalBoundsInfo {
    pub fn BeginInvoke(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
        unQuadsCount: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (pQuadsBuffer, unQuadsCount, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
        unQuadsCount: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pQuadsBuffer, unQuadsCount))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPhysicalBoundsInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPhysicalBoundsInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPlayAreaSize")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__SetWorkingPlayAreaSize {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPlayAreaSize")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPlayAreaSize => "OVR.OpenVR"
    ."IVRChaperoneSetup/_SetWorkingPlayAreaSize"
);
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPlayAreaSize")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPlayAreaSize {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPlayAreaSize")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPlayAreaSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPlayAreaSize")]
impl crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPlayAreaSize {
    pub fn BeginInvoke(
        &mut self,
        sizeX: f32,
        sizeZ: f32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (sizeX, sizeZ, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        sizeX: f32,
        sizeZ: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (sizeX, sizeZ))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingPlayAreaSize")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingPlayAreaSize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingSeatedZeroPoseToRawTrackingPose"
)]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__SetWorkingSeatedZeroPoseToRawTrackingPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingSeatedZeroPoseToRawTrackingPose"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__SetWorkingSeatedZeroPoseToRawTrackingPose =>
    "OVR.OpenVR"."IVRChaperoneSetup/_SetWorkingSeatedZeroPoseToRawTrackingPose"
);
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingSeatedZeroPoseToRawTrackingPose"
)]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingSeatedZeroPoseToRawTrackingPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingSeatedZeroPoseToRawTrackingPose"
)]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingSeatedZeroPoseToRawTrackingPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingSeatedZeroPoseToRawTrackingPose"
)]
impl crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingSeatedZeroPoseToRawTrackingPose {
    pub fn BeginInvoke(
        &mut self,
        pMatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (pMatSeatedZeroPoseToRawTrackingPose, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pMatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pMatSeatedZeroPoseToRawTrackingPose, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pMatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pMatSeatedZeroPoseToRawTrackingPose))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingSeatedZeroPoseToRawTrackingPose"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingSeatedZeroPoseToRawTrackingPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingStandingZeroPoseToRawTrackingPose"
)]
#[repr(C)]
#[derive(Debug)]
pub struct IVRChaperoneSetup__SetWorkingStandingZeroPoseToRawTrackingPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingStandingZeroPoseToRawTrackingPose"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRChaperoneSetup__SetWorkingStandingZeroPoseToRawTrackingPose =>
    "OVR.OpenVR"."IVRChaperoneSetup/_SetWorkingStandingZeroPoseToRawTrackingPose"
);
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingStandingZeroPoseToRawTrackingPose"
)]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingStandingZeroPoseToRawTrackingPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingStandingZeroPoseToRawTrackingPose"
)]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingStandingZeroPoseToRawTrackingPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingStandingZeroPoseToRawTrackingPose"
)]
impl crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingStandingZeroPoseToRawTrackingPose {
    pub fn BeginInvoke(
        &mut self,
        pMatStandingZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (pMatStandingZeroPoseToRawTrackingPose, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pMatStandingZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pMatStandingZeroPoseToRawTrackingPose, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pMatStandingZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pMatStandingZeroPoseToRawTrackingPose))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "OVR+OpenVR+IVRChaperoneSetup+_SetWorkingStandingZeroPoseToRawTrackingPose"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRChaperoneSetup__SetWorkingStandingZeroPoseToRawTrackingPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
