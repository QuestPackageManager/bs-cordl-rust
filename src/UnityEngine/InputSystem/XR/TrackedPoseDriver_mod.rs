#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackedPoseDriver {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_TrackingType: crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_TrackingType,
    pub m_UpdateType: crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_UpdateType,
    pub m_IgnoreTrackingState: bool,
    pub m_PositionInput: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_RotationInput: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_TrackingStateInput: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_CurrentPosition: crate::UnityEngine::Vector3,
    pub m_CurrentRotation: crate::UnityEngine::Quaternion,
    pub m_CurrentTrackingState: crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_TrackingStates,
    pub m_RotationBound: bool,
    pub m_PositionBound: bool,
    pub m_TrackingStateBound: bool,
    pub m_IsFirstUpdate: bool,
    pub m_PositionAction: *mut crate::UnityEngine::InputSystem::InputAction,
    pub m_RotationAction: *mut crate::UnityEngine::InputSystem::InputAction,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::TrackedPoseDriver
    => "UnityEngine.InputSystem.XR"."TrackedPoseDriver"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XR::TrackedPoseDriver {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::XR::TrackedPoseDriver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver")]
impl crate::UnityEngine::InputSystem::XR::TrackedPoseDriver {
    #[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver+TrackingStates")]
    pub type TrackingStates = crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_TrackingStates;
    #[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver+TrackingType")]
    pub type TrackingType = crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_TrackingType;
    #[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver+UpdateType")]
    pub type UpdateType = crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_UpdateType;
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
    pub fn BindActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindTrackingState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindTrackingState", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasStereoCamera(
        &mut self,
        cameraComponent: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasStereoCamera", (cameraComponent))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnBeforeRender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeRender", ())?;
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
    pub fn OnPositionCanceled(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPositionCanceled", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnPositionPerformed(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPositionPerformed", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnRotationCanceled(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRotationCanceled", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnRotationPerformed(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRotationPerformed", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnTrackingStateCanceled(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTrackingStateCanceled", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnTrackingStatePerformed(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTrackingStatePerformed", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn PerformUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadTrackingState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadTrackingState", ())?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLocalTransform(
        &mut self,
        newPosition: crate::UnityEngine::Vector3,
        newRotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalTransform", (newPosition, newRotation))?;
        Ok(__cordl_ret)
    }
    pub fn UnbindActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnbindPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnbindRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnbindTrackingState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindTrackingState", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCallback", ())?;
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
    pub fn get_ignoreTrackingState(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ignoreTrackingState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_positionAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = __cordl_object
            .invoke("get_positionAction", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_positionInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_positionInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rotationAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = __cordl_object
            .invoke("get_rotationAction", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rotationInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_rotationInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackingStateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_trackingStateInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackingType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_TrackingType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_TrackingType = __cordl_object
            .invoke("get_trackingType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_updateType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_UpdateType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_UpdateType = __cordl_object
            .invoke("get_updateType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ignoreTrackingState(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ignoreTrackingState", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_positionAction(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_positionAction", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_positionInput(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_positionInput", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rotationAction(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rotationAction", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rotationInput(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rotationInput", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_trackingStateInput(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackingStateInput", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_trackingType(
        &mut self,
        value: crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_TrackingType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackingType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_updateType(
        &mut self,
        value: crate::UnityEngine::InputSystem::XR::TrackedPoseDriver_UpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_updateType", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::XR::TrackedPoseDriver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver+TrackingStates")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackedPoseDriver_TrackingStates {
    None = 0i32,
    Position = 1i32,
    Rotation = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver+TrackingStates")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::TrackedPoseDriver_TrackingStates =>
    "UnityEngine.InputSystem.XR"."TrackedPoseDriver/TrackingStates"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver+TrackingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackedPoseDriver_TrackingType {
    PositionOnly = 2i32,
    RotationAndPosition = 0i32,
    RotationOnly = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver+TrackingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::TrackedPoseDriver_TrackingType =>
    "UnityEngine.InputSystem.XR"."TrackedPoseDriver/TrackingType"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver+UpdateType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackedPoseDriver_UpdateType {
    BeforeRender = 2i32,
    Update = 1i32,
    UpdateAndBeforeRender = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+TrackedPoseDriver+UpdateType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::TrackedPoseDriver_UpdateType =>
    "UnityEngine.InputSystem.XR"."TrackedPoseDriver/UpdateType"
);
