#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackedPoseDriver {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_Device: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_DeviceType,
    pub m_PoseSource: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
    pub m_PoseProviderComponent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::XR::Interaction::BasePoseProvider,
    >,
    pub m_TrackingType: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackingType,
    pub m_UpdateType: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_UpdateType,
    pub m_UseRelativeTransform: bool,
    pub m_OriginPose: crate::UnityEngine::Pose,
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpatialTracking::TrackedPoseDriver
    => "UnityEngine.SpatialTracking"."TrackedPoseDriver"
);
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver")]
impl std::ops::Deref for crate::UnityEngine::SpatialTracking::TrackedPoseDriver {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver")]
impl std::ops::DerefMut for crate::UnityEngine::SpatialTracking::TrackedPoseDriver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver")]
impl crate::UnityEngine::SpatialTracking::TrackedPoseDriver {
    #[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+DeviceType")]
    pub type DeviceType = crate::UnityEngine::SpatialTracking::TrackedPoseDriver_DeviceType;
    #[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+TrackedPose")]
    pub type TrackedPose = crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose;
    #[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+TrackingType")]
    pub type TrackingType = crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackingType;
    #[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+UpdateType")]
    pub type UpdateType = crate::UnityEngine::SpatialTracking::TrackedPoseDriver_UpdateType;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CacheLocalPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CacheLocalPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPoseData(
        &mut self,
        device: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_DeviceType,
        poseSource: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
        resultPose: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SpatialTracking::PoseDataFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpatialTracking::PoseDataFlags = __cordl_object
            .invoke("GetPoseData", (device, poseSource, resultPose))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasStereoCamera(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasStereoCamera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnBeforeRender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeRender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PerformUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetToCachedLocalPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetToCachedLocalPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalTransform(
        &mut self,
        newPosition: crate::UnityEngine::Vector3,
        newRotation: crate::UnityEngine::Quaternion,
        poseFlags: crate::UnityEngine::SpatialTracking::PoseDataFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalTransform", (newPosition, newRotation, poseFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPoseSource(
        &mut self,
        deviceType: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_DeviceType,
        pose: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetPoseSource", (deviceType, pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransformPoseByOriginIfNeeded(
        &mut self,
        pose: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke("TransformPoseByOriginIfNeeded", (pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseRelativeTransform(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseRelativeTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SpatialTracking::TrackedPoseDriver_DeviceType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_DeviceType = __cordl_object
            .invoke("get_deviceType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_originPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke("get_originPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_poseProviderComponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::XR::Interaction::BasePoseProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::XR::Interaction::BasePoseProvider,
        > = __cordl_object.invoke("get_poseProviderComponent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_poseSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose = __cordl_object
            .invoke("get_poseSource", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_trackingType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackingType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackingType = __cordl_object
            .invoke("get_trackingType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_updateType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SpatialTracking::TrackedPoseDriver_UpdateType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_UpdateType = __cordl_object
            .invoke("get_updateType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UseRelativeTransform(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UseRelativeTransform", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_deviceType(
        &mut self,
        value: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_DeviceType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deviceType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_originPose(
        &mut self,
        value: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_originPose", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_poseProviderComponent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::XR::Interaction::BasePoseProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_poseProviderComponent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_poseSource(
        &mut self,
        value: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_poseSource", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_trackingType(
        &mut self,
        value: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackingType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackingType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_updateType(
        &mut self,
        value: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_UpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_updateType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SpatialTracking::TrackedPoseDriver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+DeviceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackedPoseDriver_DeviceType {
    #[default]
    GenericXRController = 1i32,
    GenericXRDevice = 0i32,
    GenericXRRemote = 2i32,
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+DeviceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SpatialTracking::TrackedPoseDriver_DeviceType =>
    "UnityEngine.SpatialTracking"."TrackedPoseDriver/DeviceType"
);
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+TrackedPose")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackedPoseDriver_TrackedPose {
    #[default]
    Center = 2i32,
    ColorCamera = 6i32,
    DepthCameraDeprecated = 7i32,
    DeviceDeprecated = 9i32,
    FisheyeCameraDeprected = 8i32,
    Head = 3i32,
    LeftEye = 0i32,
    LeftPose = 4i32,
    RemotePose = 10i32,
    RightEye = 1i32,
    RightPose = 5i32,
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+TrackedPose")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose =>
    "UnityEngine.SpatialTracking"."TrackedPoseDriver/TrackedPose"
);
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+TrackingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackedPoseDriver_TrackingType {
    #[default]
    PositionOnly = 2i32,
    RotationAndPosition = 0i32,
    RotationOnly = 1i32,
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+TrackingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackingType =>
    "UnityEngine.SpatialTracking"."TrackedPoseDriver/TrackingType"
);
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+UpdateType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackedPoseDriver_UpdateType {
    #[default]
    BeforeRender = 2i32,
    Update = 1i32,
    UpdateAndBeforeRender = 0i32,
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriver+UpdateType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SpatialTracking::TrackedPoseDriver_UpdateType =>
    "UnityEngine.SpatialTracking"."TrackedPoseDriver/UpdateType"
);
