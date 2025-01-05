#[cfg(feature = "OVRComposition")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRComposition {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub cameraInTrackingSpace: bool,
    pub cameraRig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCameraRig>,
    pub usingLastAttachedNodePose: bool,
    pub lastAttachedNodePose: crate::GlobalNamespace::OVRPose,
}
#[cfg(feature = "OVRComposition")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRComposition => ""
    ."OVRComposition"
);
#[cfg(feature = "OVRComposition")]
impl std::ops::Deref for crate::GlobalNamespace::OVRComposition {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRComposition")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRComposition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRComposition")]
impl crate::GlobalNamespace::OVRComposition {
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ComputeCameraTrackingSpacePose(
        &mut self,
        extrinsics: crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPose = __cordl_object
            .invoke("ComputeCameraTrackingSpacePose", (extrinsics))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeCameraWorldSpacePose(
        &mut self,
        extrinsics: crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPose = __cordl_object
            .invoke("ComputeCameraWorldSpacePose", (extrinsics, mainCamera))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentObject, mainCamera, configuration))?;
        Ok(__cordl_object.into())
    }
    pub fn RecenterPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecenterPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshCameraRig(
        &mut self,
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshCameraRig", (parentObject, mainCamera))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
        trackingOrigin: crate::GlobalNamespace::OVRManager_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (gameObject, mainCamera, configuration, trackingOrigin))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parentObject, mainCamera, configuration))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRComposition")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRComposition {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
