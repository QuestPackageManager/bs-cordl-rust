#[cfg(feature = "OVRLocatable")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRLocatable {
    pub _Handle_k__BackingField: u64,
}
#[cfg(feature = "OVRLocatable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for OVRLocatable => ""."OVRLocatable"
);
#[cfg(feature = "OVRLocatable")]
unsafe impl quest_hook::libil2cpp::ThisArgument for OVRLocatable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRLocatable")]
impl OVRLocatable {
    #[cfg(feature = "OVRLocatable+TrackingSpacePose")]
    pub type TrackingSpacePose = crate::GlobalNamespace::OVRLocatable_TrackingSpacePose;
    pub fn get_Handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Handle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsEnabled",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Type",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_OVRLocatable0(
        &mut self,
        other: OVRLocatable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRLocatable__get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRLocatable>.get_Type",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        anchor: OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (anchor),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRLocatable__FromAnchor(
        &mut self,
        anchor: OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<OVRLocatable> {
        let __cordl_ret: OVRLocatable = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRLocatable>.FromAnchor",
            (anchor),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetEnabledAsync(
        &mut self,
        enabled: bool,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<OVRTask_1<bool>> {
        let __cordl_ret: OVRTask_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetEnabledAsync",
            (enabled, timeout),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetSceneAnchorPose(
        &mut self,
        pose: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetSceneAnchorPose",
            (pose),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetSpatialAnchorPose(
        &mut self,
        pose: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRLocatable_TrackingSpacePose,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetSpatialAnchorPose",
            (pose),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRLocatable__get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRLocatable>.get_Handle",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRLocatable+TrackingSpacePose")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRLocatable_TrackingSpacePose {
    pub _Position_k__BackingField: crate::System::Nullable_1<
        crate::UnityEngine::Vector3,
    >,
    pub _Rotation_k__BackingField: crate::System::Nullable_1<
        crate::UnityEngine::Quaternion,
    >,
    pub _flags: crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
}
#[cfg(feature = "OVRLocatable+TrackingSpacePose")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRLocatable_TrackingSpacePose
    => ""."OVRLocatable/TrackingSpacePose"
);
#[cfg(feature = "OVRLocatable+TrackingSpacePose")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRLocatable_TrackingSpacePose {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRLocatable+TrackingSpacePose")]
impl crate::GlobalNamespace::OVRLocatable_TrackingSpacePose {
    pub fn get_Rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::Quaternion>,
    > {
        let __cordl_ret: crate::System::Nullable_1<crate::UnityEngine::Quaternion> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Rotation",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::Vector3>,
    > {
        let __cordl_ret: crate::System::Nullable_1<crate::UnityEngine::Vector3> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Position",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsRotationTracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsRotationTracked",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        flags: crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (position, rotation, flags),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ComputeWorldPosition(
        &mut self,
        camera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::Vector3>,
    > {
        let __cordl_ret: crate::System::Nullable_1<crate::UnityEngine::Vector3> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ComputeWorldPosition",
            (camera),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ComputeWorldRotation(
        &mut self,
        camera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::Quaternion>,
    > {
        let __cordl_ret: crate::System::Nullable_1<crate::UnityEngine::Quaternion> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ComputeWorldRotation",
            (camera),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPositionTracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsPositionTracked",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
