#[cfg(feature = "PSVR2Helper")]
#[repr(C)]
#[derive(Debug)]
pub struct PSVR2Helper {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _leftController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::UnityXRController_Configuration,
    >,
    pub _rightController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::UnityXRController_Configuration,
    >,
    pub _pauseGameActionReference: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionReference,
    >,
    pub _defaultPose: crate::UnityEngine::Pose,
    pub _controllers: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::XR::XRNode,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UnityXRController>,
    >,
    pub _controllersWithRumble: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::XR::XRNode,
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::XR::XRControllerWithRumble,
        >,
    >,
    pub _pauseGameAction: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub _timeWhenStartedPlayingHaptic: f32,
    pub _lastTimeWhenTriggeredHaptic: f32,
    pub _hasInputFocus: bool,
    pub _menuButtonDown: bool,
    pub _menuButtonDownThisFrame: bool,
    pub _maximumHapticFrequencyLimits: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit,
    >,
    pub controllersDidChangeReferenceEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub vrControllersDisconnectedOnStartupEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub _hasVrFocus: bool,
    pub inputFocusWasCapturedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub inputFocusWasReleasedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub vrFocusWasCapturedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub vrFocusWasReleasedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub hmdUnmountedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub hmdMountedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub controllersDidDisconnectEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "PSVR2Helper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PSVR2Helper => ""."PSVR2Helper"
);
#[cfg(feature = "PSVR2Helper")]
impl std::ops::Deref for crate::GlobalNamespace::PSVR2Helper {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PSVR2Helper")]
impl std::ops::DerefMut for crate::GlobalNamespace::PSVR2Helper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PSVR2Helper")]
impl crate::GlobalNamespace::PSVR2Helper {
    pub const kLeftControllerName: &'static str = "PSVR2ControllerLeft";
    pub const kMaxHMDFrequency: i32 = 25i32;
    pub const kRightControllerName: &'static str = "PSVR2ControllerRight";
    pub const kRumbleMinimalDuration: f32 = 0.05f32;
    #[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
    pub type HeadsetHapticFrequencyLimit = crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit;
    pub fn AddControllerToMap(
        &mut self,
        forNode: crate::UnityEngine::XR::XRNode,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddControllerToMap", (forNode, device))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn CheckControllerConnectionOnStartup(
        &mut self,
        delayInSeconds: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke("CheckControllerConnectionOnStartup", (delayInSeconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAnyJoystickMaxAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetAnyJoystickMaxAxis", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMenuButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetMenuButton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMenuButtonDown(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetMenuButtonDown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePose(
        &mut self,
        nodeType: crate::UnityEngine::XR::XRNode,
        idx: i32,
        pos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rot: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetNodePose", (nodeType, idx, pos, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootPositionOffsetForLegacyNodePose(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke("GetRootPositionOffsetForLegacyNodePose", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTRCCompliantHeadsetHapticFrequency(
        &mut self,
        initialFrequency: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetTRCCompliantHeadsetHapticFrequency", (initialFrequency))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetThumbstickValue(
        &mut self,
        nodeType: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetThumbstickValue", (nodeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTriggerValue(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetTriggerValue", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplicationInputFocusLost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleApplicationInputFocusLost", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplicationInputFocusResumed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleApplicationInputFocusResumed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplicationVRFocusLost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleApplicationVRFocusLost", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplicationVRFocusResumed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleApplicationVRFocusResumed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAnyVRControllerConnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasAnyVRControllerConnected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InputDeviceChangeTriggered(
        &mut self,
        inputDevice: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        >,
        inputDeviceChange: crate::UnityEngine::InputSystem::InputDeviceChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InputDeviceChangeTriggered", (inputDevice, inputDeviceChange))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAdvancedHapticsSupported(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsAdvancedHapticsSupported", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn OnPauseGameCancelled(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPauseGameCancelled", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPauseGamePerformed(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPauseGamePerformed", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshControllersReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshControllersReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopHaptics(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopHaptics", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerHapticPulse(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        duration: f32,
        strength: f32,
        frequency: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerHapticPulse", (node, duration, strength, frequency))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetLegacyPoseOffsetForNode(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetLegacyPoseOffsetForNode", (node, position, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPoseOffsetForNode(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        poseOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetPoseOffsetForNode", (node, poseOffset))?;
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
    pub fn add_controllersDidChangeReferenceEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_controllersDidChangeReferenceEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_controllersDidDisconnectEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_controllersDidDisconnectEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_hmdMountedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_hmdMountedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_hmdUnmountedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_hmdUnmountedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_inputFocusWasCapturedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_inputFocusWasCapturedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_inputFocusWasReleasedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_inputFocusWasReleasedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_vrControllersDisconnectedOnStartupEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_vrControllersDisconnectedOnStartupEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_vrFocusWasCapturedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_vrFocusWasCapturedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_vrFocusWasReleasedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_vrFocusWasReleasedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasInputFocus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasInputFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasVrFocus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasVrFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isAlwaysWireless(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isAlwaysWireless", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vrPlatformSDK(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::VRPlatformSDK> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::VRPlatformSDK = __cordl_object
            .invoke("get_vrPlatformSDK", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_controllersDidChangeReferenceEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_controllersDidChangeReferenceEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_controllersDidDisconnectEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_controllersDidDisconnectEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_hmdMountedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_hmdMountedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_hmdUnmountedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_hmdUnmountedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_inputFocusWasCapturedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_inputFocusWasCapturedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_inputFocusWasReleasedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_inputFocusWasReleasedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_vrControllersDisconnectedOnStartupEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_vrControllersDisconnectedOnStartupEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_vrFocusWasCapturedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_vrFocusWasCapturedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_vrFocusWasReleasedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_vrFocusWasReleasedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PSVR2Helper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PSVR2Helper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PSVR2Helper")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IVRPlatformHelper>>
for crate::GlobalNamespace::PSVR2Helper {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IVRPlatformHelper> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PSVR2Helper")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IVRPlatformHelper>>
for crate::GlobalNamespace::PSVR2Helper {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IVRPlatformHelper> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PSVR2Helper_HeadsetHapticFrequencyLimit {
    pub maxTimePlayed: f32,
    pub maximumFrequency: i32,
}
#[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit => ""
    ."PSVR2Helper/HeadsetHapticFrequencyLimit"
);
#[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
impl crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit {
    pub fn _ctor(
        &mut self,
        maxTimePlayed: f32,
        maximumFrequency: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (maxTimePlayed, maximumFrequency),
        )?;
        Ok(__cordl_ret.into())
    }
}
