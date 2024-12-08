#[cfg(feature = "FirstPersonFlyingController")]
#[repr(C)]
#[derive(Debug)]
pub struct FirstPersonFlyingController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _moveSensitivity: f32,
    pub _transform: *mut crate::UnityEngine::Transform,
    pub _camera: *mut crate::UnityEngine::Camera,
    pub _cameraFov: f32,
    pub _centerAdjust: *mut crate::GlobalNamespace::VRCenterAdjust,
    pub _controller0: *mut crate::GlobalNamespace::VRController,
    pub _controller1: *mut crate::GlobalNamespace::VRController,
    pub _controllerModels: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::GameObject,
    >,
    pub _mouseLook: *mut crate::GlobalNamespace::MouseLook,
    pub _trackedPoseDriver: *mut crate::UnityEngine::SpatialTracking::TrackedPoseDriver,
    pub _cameraTransform: *mut crate::UnityEngine::Transform,
    pub _originalStereoTargetEyeMask: crate::UnityEngine::StereoTargetEyeMask,
    pub _originalCameraFov: f32,
}
#[cfg(feature = "FirstPersonFlyingController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FirstPersonFlyingController =>
    ""."FirstPersonFlyingController"
);
#[cfg(feature = "FirstPersonFlyingController")]
impl std::ops::Deref for crate::GlobalNamespace::FirstPersonFlyingController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FirstPersonFlyingController")]
impl std::ops::DerefMut for crate::GlobalNamespace::FirstPersonFlyingController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FirstPersonFlyingController")]
impl crate::GlobalNamespace::FirstPersonFlyingController {
    pub fn Inject(
        &mut self,
        camera: *mut crate::UnityEngine::Camera,
        centerAdjust: *mut crate::GlobalNamespace::VRCenterAdjust,
        controller0: *mut crate::GlobalNamespace::VRController,
        controller1: *mut crate::GlobalNamespace::VRController,
        trackedPoseDriver: *mut crate::UnityEngine::SpatialTracking::TrackedPoseDriver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Inject",
                (camera, centerAdjust, controller0, controller1, trackedPoseDriver),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FirstPersonFlyingController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FirstPersonFlyingController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
