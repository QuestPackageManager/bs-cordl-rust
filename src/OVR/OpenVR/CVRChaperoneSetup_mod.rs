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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EChaperoneConfigFile),
                bool,
                1usize,
            >("CommitWorkingCopy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CommitWorkingCopy", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (configFile)) };
        Ok(__cordl_ret.into())
    }
    pub fn ExportLiveToBuffer(
        &mut self,
        pBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        pnBufferLength: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    quest_hook::libil2cpp::ByRefMut<u32>,
                ),
                bool,
                2usize,
            >("ExportLiveToBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ExportLiveToBuffer", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pBuffer, pnBufferLength))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
                    >,
                >),
                bool,
                1usize,
            >("GetLiveCollisionBoundsInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLiveCollisionBoundsInfo", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pQuadsBuffer)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetLiveCollisionBoundsTagsInfo(
        &mut self,
        pTagsBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                >),
                bool,
                1usize,
            >("GetLiveCollisionBoundsTagsInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLiveCollisionBoundsTagsInfo", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pTagsBuffer)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
                    >,
                >),
                bool,
                1usize,
            >("GetLivePhysicalBoundsInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLivePhysicalBoundsInfo", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pQuadsBuffer)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetLiveSeatedZeroPoseToRawTrackingPose(
        &mut self,
        pmatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>),
                bool,
                1usize,
            >("GetLiveSeatedZeroPoseToRawTrackingPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLiveSeatedZeroPoseToRawTrackingPose", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pmatSeatedZeroPoseToRawTrackingPose))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
                    >,
                >),
                bool,
                1usize,
            >("GetWorkingCollisionBoundsInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetWorkingCollisionBoundsInfo", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pQuadsBuffer)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetWorkingPlayAreaRect(
        &mut self,
        rect: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdQuad_t>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdQuad_t>),
                bool,
                1usize,
            >("GetWorkingPlayAreaRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetWorkingPlayAreaRect", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (rect)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetWorkingPlayAreaSize(
        &mut self,
        pSizeX: quest_hook::libil2cpp::ByRefMut<f32>,
        pSizeZ: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<f32>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                bool,
                2usize,
            >("GetWorkingPlayAreaSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetWorkingPlayAreaSize", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pSizeX, pSizeZ))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWorkingSeatedZeroPoseToRawTrackingPose(
        &mut self,
        pmatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>),
                bool,
                1usize,
            >("GetWorkingSeatedZeroPoseToRawTrackingPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetWorkingSeatedZeroPoseToRawTrackingPose", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pmatSeatedZeroPoseToRawTrackingPose))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWorkingStandingZeroPoseToRawTrackingPose(
        &mut self,
        pmatStandingZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>),
                bool,
                1usize,
            >("GetWorkingStandingZeroPoseToRawTrackingPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetWorkingStandingZeroPoseToRawTrackingPose", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pmatStandingZeroPoseToRawTrackingPose))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ImportFromBufferToWorking(
        &mut self,
        pBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nImportFlags: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, u32),
                bool,
                2usize,
            >("ImportFromBufferToWorking")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ImportFromBufferToWorking", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pBuffer, nImportFlags))
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
    pub fn ReloadFromDisk(
        &mut self,
        configFile: crate::OVR::OpenVR::EChaperoneConfigFile,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EChaperoneConfigFile),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReloadFromDisk")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReloadFromDisk", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (configFile))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RevertWorkingCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RevertWorkingCopy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RevertWorkingCopy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWorkingCollisionBoundsInfo(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetWorkingCollisionBoundsInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetWorkingCollisionBoundsInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pQuadsBuffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWorkingCollisionBoundsTagsInfo(
        &mut self,
        pTagsBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetWorkingCollisionBoundsTagsInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetWorkingCollisionBoundsTagsInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pTagsBuffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWorkingPhysicalBoundsInfo(
        &mut self,
        pQuadsBuffer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::HmdQuad_t>,
                >),
                bool,
                1usize,
            >("SetWorkingPhysicalBoundsInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetWorkingPhysicalBoundsInfo", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pQuadsBuffer)) };
        Ok(__cordl_ret.into())
    }
    pub fn SetWorkingPlayAreaSize(
        &mut self,
        sizeX: f32,
        sizeZ: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetWorkingPlayAreaSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetWorkingPlayAreaSize", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sizeX, sizeZ))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWorkingSeatedZeroPoseToRawTrackingPose(
        &mut self,
        pMatSeatedZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetWorkingSeatedZeroPoseToRawTrackingPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetWorkingSeatedZeroPoseToRawTrackingPose", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pMatSeatedZeroPoseToRawTrackingPose))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWorkingStandingZeroPoseToRawTrackingPose(
        &mut self,
        pMatStandingZeroPoseToRawTrackingPose: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetWorkingStandingZeroPoseToRawTrackingPose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetWorkingStandingZeroPoseToRawTrackingPose", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pMatStandingZeroPoseToRawTrackingPose))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pInterface))
        };
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
