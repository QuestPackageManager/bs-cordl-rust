#[cfg(feature = "OVRCameraRig")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRCameraRig {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _trackingSpace_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _leftEyeAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _centerEyeAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _rightEyeAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _leftHandAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _rightHandAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _leftHandAnchorDetached_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _rightHandAnchorDetached_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _leftControllerInHandAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _leftHandOnControllerAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _rightControllerInHandAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _rightHandOnControllerAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _leftControllerAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _rightControllerAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _trackerAnchor_k__BackingField: *mut crate::UnityEngine::Transform,
    pub UpdatedAnchors: *mut crate::System::Action_1<*mut OVRCameraRig>,
    pub TrackingSpaceChanged: *mut crate::System::Action_1<
        *mut crate::UnityEngine::Transform,
    >,
    pub usePerEyeCameras: bool,
    pub useFixedUpdateForTracking: bool,
    pub disableEyeAnchorCameras: bool,
    pub _skipUpdate: bool,
    pub trackingSpaceName: *mut crate::System::String,
    pub trackerAnchorName: *mut crate::System::String,
    pub leftEyeAnchorName: *mut crate::System::String,
    pub centerEyeAnchorName: *mut crate::System::String,
    pub rightEyeAnchorName: *mut crate::System::String,
    pub leftHandAnchorName: *mut crate::System::String,
    pub rightHandAnchorName: *mut crate::System::String,
    pub leftControllerAnchorName: *mut crate::System::String,
    pub rightControllerAnchorName: *mut crate::System::String,
    pub leftHandAnchorDetachedName: *mut crate::System::String,
    pub rightHandAnchorDetachedName: *mut crate::System::String,
    pub leftControllerInHandAnchorName: *mut crate::System::String,
    pub leftHandOnControllerAnchorName: *mut crate::System::String,
    pub rightControllerInHandAnchorName: *mut crate::System::String,
    pub rightHandOnControllerAnchorName: *mut crate::System::String,
    pub _centerEyeCamera: *mut crate::UnityEngine::Camera,
    pub _leftEyeCamera: *mut crate::UnityEngine::Camera,
    pub _rightEyeCamera: *mut crate::UnityEngine::Camera,
    pub _previousTrackingSpaceTransform: crate::UnityEngine::Matrix4x4,
}
#[cfg(feature = "OVRCameraRig")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRCameraRig => ""."OVRCameraRig"
);
#[cfg(feature = "OVRCameraRig")]
impl std::ops::Deref for OVRCameraRig {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCameraRig")]
impl std::ops::DerefMut for OVRCameraRig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCameraRig")]
impl OVRCameraRig {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckForAnchorsInParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckForAnchorsInParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckForTrackingSpaceChangesAndRaiseEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckForTrackingSpaceChangesAndRaiseEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn ComputeTrackReferenceMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("ComputeTrackReferenceMatrix", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAnchor(
        &mut self,
        root: *mut crate::UnityEngine::Transform,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("ConfigureAnchor", (root, name))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureGameObjectIntegrity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureGameObjectIntegrity", ())?;
        Ok(__cordl_ret)
    }
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnBeforeRenderCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeRenderCallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn RaiseUpdatedAnchorsEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseUpdatedAnchorsEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn UpdateAnchors(
        &mut self,
        updateEyeAnchors: bool,
        updateHandAnchors: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAnchors", (updateEyeAnchors, updateHandAnchors))?;
        Ok(__cordl_ret)
    }
    pub fn _CheckForAnchorsInParent_g__Check_105_0<T>(
        &mut self,
        node: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<CheckForAnchorsInParent>g__Check|105_0", (node))?;
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
    pub fn add_TrackingSpaceChanged(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_TrackingSpaceChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_UpdatedAnchors(
        &mut self,
        value: *mut crate::System::Action_1<*mut OVRCameraRig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_UpdatedAnchors", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_centerEyeAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_centerEyeAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftControllerAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_leftControllerAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftControllerInHandAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_leftControllerInHandAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftEyeAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_leftEyeAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftEyeCamera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Camera> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Camera = __cordl_object
            .invoke("get_leftEyeCamera", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftHandAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_leftHandAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftHandAnchorDetached(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_leftHandAnchorDetached", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftHandOnControllerAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_leftHandOnControllerAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightControllerAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_rightControllerAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightControllerInHandAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_rightControllerInHandAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightEyeAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_rightEyeAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightEyeCamera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Camera> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Camera = __cordl_object
            .invoke("get_rightEyeCamera", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightHandAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_rightHandAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightHandAnchorDetached(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_rightHandAnchorDetached", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightHandOnControllerAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_rightHandOnControllerAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackerAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_trackerAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackingSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_trackingSpace", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_TrackingSpaceChanged(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_TrackingSpaceChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_UpdatedAnchors(
        &mut self,
        value: *mut crate::System::Action_1<*mut OVRCameraRig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_UpdatedAnchors", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_centerEyeAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_centerEyeAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_leftControllerAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftControllerAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_leftControllerInHandAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftControllerInHandAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_leftEyeAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftEyeAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_leftHandAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftHandAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_leftHandAnchorDetached(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftHandAnchorDetached", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_leftHandOnControllerAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftHandOnControllerAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rightControllerAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightControllerAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rightControllerInHandAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightControllerInHandAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rightEyeAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightEyeAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rightHandAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightHandAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rightHandAnchorDetached(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightHandAnchorDetached", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rightHandOnControllerAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightHandOnControllerAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_trackerAnchor(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackerAnchor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_trackingSpace(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackingSpace", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRCameraRig")]
impl quest_hook::libil2cpp::ObjectType for OVRCameraRig {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}