#[cfg(feature = "OVR+OpenVR+CVRChaperoneSetup")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRChaperoneSetup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRChaperoneSetup,
}
#[cfg(feature = "OVR+OpenVR+CVRChaperoneSetup")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRChaperoneSetup {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRChaperoneSetup";
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
#[cfg(feature = "OVR+OpenVR+CVRChaperoneSetup")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRChaperoneSetup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CommitWorkingCopy(
        &mut self,
        configFile: crate::OVR::OpenVR::EChaperoneConfigFile,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CommitWorkingCopy", (configFile))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExportLiveToBuffer(
        &mut self,
        pBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        pnBufferLength: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ExportLiveToBuffer", (pBuffer, pnBufferLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLiveCollisionBoundsInfo(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetLiveCollisionBoundsInfo", (pQuadsBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLiveCollisionBoundsTagsInfo(
        &mut self,
        pTagsBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetLiveCollisionBoundsTagsInfo", (pTagsBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLivePhysicalBoundsInfo(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetLivePhysicalBoundsInfo", (pQuadsBuffer))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetWorkingCollisionBoundsInfo(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetWorkingCollisionBoundsInfo", (pQuadsBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWorkingPlayAreaRect(
        &mut self,
        rect: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdQuad_t>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetWorkingPlayAreaRect", (rect))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ImportFromBufferToWorking(
        &mut self,
        pBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nImportFlags: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ImportFromBufferToWorking", (pBuffer, nImportFlags))?;
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
    pub fn ReloadFromDisk(
        &mut self,
        configFile: crate::OVR::OpenVR::EChaperoneConfigFile,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadFromDisk", (configFile))?;
        Ok(__cordl_ret.into())
    }
    pub fn RevertWorkingCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RevertWorkingCopy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWorkingCollisionBoundsInfo(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWorkingCollisionBoundsInfo", (pQuadsBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWorkingCollisionBoundsTagsInfo(
        &mut self,
        pTagsBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWorkingCollisionBoundsTagsInfo", (pTagsBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWorkingPhysicalBoundsInfo(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetWorkingPhysicalBoundsInfo", (pQuadsBuffer))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "OVR+OpenVR+CVRChaperoneSetup")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRChaperoneSetup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
