#[cfg(feature = "cordl_class_OVR+OpenVR+CVRSpatialAnchors")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRSpatialAnchors {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRSpatialAnchors,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRSpatialAnchors")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRSpatialAnchors {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRSpatialAnchors";
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
#[cfg(feature = "OVR+OpenVR+CVRSpatialAnchors")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRSpatialAnchors {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSpatialAnchors")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRSpatialAnchors {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        crate::OVR::OpenVR::EVRSpatialAnchorError,
                        2usize,
                    >("CreateSpatialAnchorFromDescriptor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSpatialAnchorFromDescriptor", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = unsafe {
            cordl_method_info.invoke_unchecked(self, (pchDescriptor, pHandleOut))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpatialAnchorFromPose(
        &mut self,
        unDeviceIndex: u32,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pPose: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::SpatialAnchorPose_t>,
        pHandleOut: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::OVR::OpenVR::ETrackingUniverseOrigin,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::SpatialAnchorPose_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        crate::OVR::OpenVR::EVRSpatialAnchorError,
                        4usize,
                    >("CreateSpatialAnchorFromPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSpatialAnchorFromPose", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (unDeviceIndex, eOrigin, pPose, pHandleOut))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSpatialAnchorDescriptor(
        &mut self,
        unHandle: u32,
        pchDescriptorOut: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        punDescriptorBufferLenInOut: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        crate::OVR::OpenVR::EVRSpatialAnchorError,
                        3usize,
                    >("GetSpatialAnchorDescriptor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSpatialAnchorDescriptor", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (unHandle, pchDescriptorOut, punDescriptorBufferLenInOut),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::OVR::OpenVR::ETrackingUniverseOrigin,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::SpatialAnchorPose_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVRSpatialAnchorError,
                        3usize,
                    >("GetSpatialAnchorPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSpatialAnchorPose", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = unsafe {
            cordl_method_info.invoke_unchecked(self, (unHandle, eOrigin, pPoseOut))?
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
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRSpatialAnchors")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRSpatialAnchors {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
