#[cfg(feature = "OVR+OpenVR+CVRChaperoneSetup")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRChaperoneSetup {
    __cordl_parent: crate::System::Object,
    pub FnTable: crate::OVR::OpenVR::IVRChaperoneSetup,
}
#[cfg(feature = "OVR+OpenVR+CVRChaperoneSetup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRChaperoneSetup => "OVR.OpenVR"
    ."CVRChaperoneSetup"
);
#[cfg(feature = "OVR+OpenVR+CVRChaperoneSetup")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRChaperoneSetup {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRChaperoneSetup")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRChaperoneSetup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRChaperoneSetup")]
impl crate::OVR::OpenVR::CVRChaperoneSetup {
    pub fn SetWorkingSeatedZeroPoseToRawTrackingPose(
        &mut self,
        pMatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetWorkingSeatedZeroPoseToRawTrackingPose",
                (pMatSeatedZeroPoseToRawTrackingPose),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReloadFromDisk(
        &mut self,
        configFile: crate::OVR::OpenVR::EChaperoneConfigFile,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadFromDisk", (configFile))?;
        Ok(__cordl_ret)
    }
    pub fn GetWorkingStandingZeroPoseToRawTrackingPose(
        &mut self,
        pmatStandingZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetWorkingStandingZeroPoseToRawTrackingPose",
                (pmatStandingZeroPoseToRawTrackingPose),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetWorkingCollisionBoundsTagsInfo(
        &mut self,
        pTagsBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWorkingCollisionBoundsTagsInfo", (pTagsBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn SetWorkingStandingZeroPoseToRawTrackingPose(
        &mut self,
        pMatStandingZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetWorkingStandingZeroPoseToRawTrackingPose",
                (pMatStandingZeroPoseToRawTrackingPose),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetWorkingSeatedZeroPoseToRawTrackingPose(
        &mut self,
        pmatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetWorkingSeatedZeroPoseToRawTrackingPose",
                (pmatSeatedZeroPoseToRawTrackingPose),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetWorkingPlayAreaSize(
        &mut self,
        pSizeX: quest_hook::libil2cpp::ByRefMut<f32>,
        pSizeZ: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetWorkingPlayAreaSize", (pSizeX, pSizeZ))?;
        Ok(__cordl_ret)
    }
    pub fn GetLiveCollisionBoundsInfo(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetLiveCollisionBoundsInfo", (pQuadsBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn ExportLiveToBuffer(
        &mut self,
        pBuffer: *mut crate::System::Text::StringBuilder,
        pnBufferLength: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ExportLiveToBuffer", (pBuffer, pnBufferLength))?;
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
    pub fn SetWorkingCollisionBoundsInfo(
        &mut self,
        pQuadsBuffer: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::HmdQuad_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWorkingCollisionBoundsInfo", (pQuadsBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn SetWorkingPlayAreaSize(
        &mut self,
        sizeX: f32,
        sizeZ: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWorkingPlayAreaSize", (sizeX, sizeZ))?;
        Ok(__cordl_ret)
    }
    pub fn ImportFromBufferToWorking(
        &mut self,
        pBuffer: *mut crate::System::String,
        nImportFlags: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ImportFromBufferToWorking", (pBuffer, nImportFlags))?;
        Ok(__cordl_ret)
    }
    pub fn RevertWorkingCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RevertWorkingCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetWorkingPlayAreaRect(
        &mut self,
        rect: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdQuad_t>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetWorkingPlayAreaRect", (rect))?;
        Ok(__cordl_ret)
    }
    pub fn GetWorkingCollisionBoundsInfo(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetWorkingCollisionBoundsInfo", (pQuadsBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn GetLiveSeatedZeroPoseToRawTrackingPose(
        &mut self,
        pmatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetLiveSeatedZeroPoseToRawTrackingPose",
                (pmatSeatedZeroPoseToRawTrackingPose),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetLivePhysicalBoundsInfo(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetLivePhysicalBoundsInfo", (pQuadsBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn CommitWorkingCopy(
        &mut self,
        configFile: crate::OVR::OpenVR::EChaperoneConfigFile,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CommitWorkingCopy", (configFile))?;
        Ok(__cordl_ret)
    }
    pub fn GetLiveCollisionBoundsTagsInfo(
        &mut self,
        pTagsBuffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetLiveCollisionBoundsTagsInfo", (pTagsBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn SetWorkingPhysicalBoundsInfo(
        &mut self,
        pQuadsBuffer: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::HmdQuad_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetWorkingPhysicalBoundsInfo", (pQuadsBuffer))?;
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
#[cfg(feature = "OVR+OpenVR+CVRChaperoneSetup")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRChaperoneSetup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
