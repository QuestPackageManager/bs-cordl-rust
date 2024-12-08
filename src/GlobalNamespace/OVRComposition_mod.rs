#[cfg(feature = "OVRComposition")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRComposition {
    __cordl_parent: crate::System::Object,
    pub cameraInTrackingSpace: bool,
    pub cameraRig: *mut OVRCameraRig,
    pub usingLastAttachedNodePose: bool,
    pub lastAttachedNodePose: OVRPose,
}
#[cfg(feature = "OVRComposition")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRComposition => ""."OVRComposition"
);
#[cfg(feature = "OVRComposition")]
impl std::ops::Deref for OVRComposition {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRComposition")]
impl std::ops::DerefMut for OVRComposition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRComposition")]
impl OVRComposition {
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompositionMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_CompositionMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CompositionMethod = __cordl_object
            .invoke("CompositionMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn ComputeCameraTrackingSpacePose(
        &mut self,
        extrinsics: crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
    ) -> quest_hook::libil2cpp::Result<OVRPose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: OVRPose = __cordl_object
            .invoke("ComputeCameraTrackingSpacePose", (extrinsics))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeCameraWorldSpacePose(
        &mut self,
        extrinsics: crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
        mainCamera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<OVRPose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: OVRPose = __cordl_object
            .invoke("ComputeCameraWorldSpacePose", (extrinsics, mainCamera))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parentObject: *mut crate::UnityEngine::GameObject,
        mainCamera: *mut crate::UnityEngine::Camera,
        configuration: *mut OVRMixedRealityCaptureConfiguration,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentObject, mainCamera, configuration))?;
        Ok(__cordl_object)
    }
    pub fn RecenterPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecenterPose", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshCameraRig(
        &mut self,
        parentObject: *mut crate::UnityEngine::GameObject,
        mainCamera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshCameraRig", (parentObject, mainCamera))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
        mainCamera: *mut crate::UnityEngine::Camera,
        configuration: *mut OVRMixedRealityCaptureConfiguration,
        trackingOrigin: crate::GlobalNamespace::OVRManager_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (gameObject, mainCamera, configuration, trackingOrigin))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        parentObject: *mut crate::UnityEngine::GameObject,
        mainCamera: *mut crate::UnityEngine::Camera,
        configuration: *mut OVRMixedRealityCaptureConfiguration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parentObject, mainCamera, configuration))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRComposition")]
impl quest_hook::libil2cpp::ObjectType for OVRComposition {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
