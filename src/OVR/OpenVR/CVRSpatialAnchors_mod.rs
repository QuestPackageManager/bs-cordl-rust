#[cfg(feature = "OVR+OpenVR+CVRSpatialAnchors")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRSpatialAnchors {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub FnTable: crate::OVR::OpenVR::IVRSpatialAnchors,
}
#[cfg(feature = "OVR+OpenVR+CVRSpatialAnchors")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRSpatialAnchors => "OVR.OpenVR"
    ."CVRSpatialAnchors"
);
#[cfg(feature = "OVR+OpenVR+CVRSpatialAnchors")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRSpatialAnchors {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSpatialAnchors")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRSpatialAnchors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSpatialAnchors")]
impl crate::OVR::OpenVR::CVRSpatialAnchors {
    pub fn CreateSpatialAnchorFromDescriptor(
        &mut self,
        pchDescriptor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pHandleOut: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke("CreateSpatialAnchorFromDescriptor", (pchDescriptor, pHandleOut))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpatialAnchorFromPose(
        &mut self,
        unDeviceIndex: u32,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pPose: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::SpatialAnchorPose_t>,
        pHandleOut: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke(
                "CreateSpatialAnchorFromPose",
                (unDeviceIndex, eOrigin, pPose, pHandleOut),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpatialAnchorDescriptor(
        &mut self,
        unHandle: u32,
        pchDescriptorOut: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        punDescriptorBufferLenInOut: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke(
                "GetSpatialAnchorDescriptor",
                (unHandle, pchDescriptorOut, punDescriptorBufferLenInOut),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpatialAnchorPose(
        &mut self,
        unHandle: u32,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pPoseOut: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::SpatialAnchorPose_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke("GetSpatialAnchorPose", (unHandle, eOrigin, pPoseOut))?;
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
#[cfg(feature = "OVR+OpenVR+CVRSpatialAnchors")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRSpatialAnchors {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
