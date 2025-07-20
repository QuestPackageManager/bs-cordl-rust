#[cfg(feature = "PSVR2Helper")]
#[repr(C)]
#[derive(Debug)]
pub struct PSVR2Helper {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
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
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::XR::XRNode,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UnityXRController>,
        >,
    >,
    pub _controllersWithRumble: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::XR::XRNode,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::XR::XRControllerWithRumble,
            >,
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
        crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit,
        >,
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
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PSVR2Helper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PSVR2Helper";
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
#[cfg(feature = "PSVR2Helper")]
impl std::ops::Deref for crate::GlobalNamespace::PSVR2Helper {
    type Target = crate::UnityEngine::MonoBehaviour;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::XR::XRNode,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputDevice,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddControllerToMap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "AddControllerToMap", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (forNode, device))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "Awake", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckControllerConnectionOnStartup(
        &mut self,
        delayInSeconds: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                1usize,
            >("CheckControllerConnectionOnStartup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "CheckControllerConnectionOnStartup", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, (delayInSeconds))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAnyJoystickMaxAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector2,
                0usize,
            >("GetAnyJoystickMaxAxis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "GetAnyJoystickMaxAxis", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMenuButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("GetMenuButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "GetMenuButton", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMenuButtonDown(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("GetMenuButtonDown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "GetMenuButtonDown", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePose(
        &mut self,
        nodeType: crate::UnityEngine::XR::XRNode,
        idx: i32,
        pos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rot: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::XR::XRNode,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
                ),
                bool,
                4usize,
            >("GetNodePose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "GetNodePose", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (nodeType, idx, pos, rot))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRootPositionOffsetForLegacyNodePose(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::XR::XRNode),
                crate::UnityEngine::Pose,
                1usize,
            >("GetRootPositionOffsetForLegacyNodePose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "GetRootPositionOffsetForLegacyNodePose", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Pose = unsafe {
            method.invoke_unchecked(self, (node))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTRCCompliantHeadsetHapticFrequency(
        &mut self,
        initialFrequency: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), i32, 1usize>("GetTRCCompliantHeadsetHapticFrequency")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "GetTRCCompliantHeadsetHapticFrequency", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (initialFrequency))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetThumbstickValue(
        &mut self,
        nodeType: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::XR::XRNode),
                crate::UnityEngine::Vector2,
                1usize,
            >("GetThumbstickValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "GetThumbstickValue", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, (nodeType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTriggerValue(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::XR::XRNode),
                f32,
                1usize,
            >("GetTriggerValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "GetTriggerValue", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplicationInputFocusLost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleApplicationInputFocusLost")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "HandleApplicationInputFocusLost", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplicationInputFocusResumed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleApplicationInputFocusResumed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "HandleApplicationInputFocusResumed", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplicationVRFocusLost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleApplicationVRFocusLost")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "HandleApplicationVRFocusLost", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleApplicationVRFocusResumed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleApplicationVRFocusResumed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "HandleApplicationVRFocusResumed", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasAnyVRControllerConnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("HasAnyVRControllerConnected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "HasAnyVRControllerConnected", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InputDeviceChangeTriggered(
        &mut self,
        inputDevice: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        >,
        inputDeviceChange: crate::UnityEngine::InputSystem::InputDeviceChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputDevice,
                    >,
                    crate::UnityEngine::InputSystem::InputDeviceChange,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("InputDeviceChangeTriggered")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "InputDeviceChangeTriggered", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inputDevice, inputDeviceChange))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsAdvancedHapticsSupported(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::XR::XRNode),
                bool,
                1usize,
            >("IsAdvancedHapticsSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "IsAdvancedHapticsSupported", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("LateUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "LateUpdate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "OnDestroy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPauseGameCancelled(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::InputAction_CallbackContext),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnPauseGameCancelled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "OnPauseGameCancelled", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPauseGamePerformed(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::InputAction_CallbackContext),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnPauseGamePerformed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "OnPauseGamePerformed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RefreshControllersReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RefreshControllersReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "RefreshControllersReference", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StopHaptics(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::XR::XRNode),
                quest_hook::libil2cpp::Void,
                1usize,
            >("StopHaptics")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "StopHaptics", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TriggerHapticPulse(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        duration: f32,
        strength: f32,
        frequency: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::XR::XRNode, f32, f32, f32),
                quest_hook::libil2cpp::Void,
                4usize,
            >("TriggerHapticPulse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "TriggerHapticPulse", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, duration, strength, frequency))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetLegacyPoseOffsetForNode(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::XR::XRNode,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                bool,
                3usize,
            >("TryGetLegacyPoseOffsetForNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "TryGetLegacyPoseOffsetForNode", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (node, position, rotation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPoseOffsetForNode(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        poseOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::XR::XRNode,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
                ),
                bool,
                2usize,
            >("TryGetPoseOffsetForNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "TryGetPoseOffsetForNode", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (node, poseOffset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_controllersDidChangeReferenceEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_controllersDidChangeReferenceEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "add_controllersDidChangeReferenceEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_controllersDidDisconnectEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_controllersDidDisconnectEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "add_controllersDidDisconnectEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_hmdMountedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_hmdMountedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "add_hmdMountedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_hmdUnmountedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_hmdUnmountedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "add_hmdUnmountedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_inputFocusWasCapturedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_inputFocusWasCapturedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "add_inputFocusWasCapturedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_inputFocusWasReleasedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_inputFocusWasReleasedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "add_inputFocusWasReleasedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_vrControllersDisconnectedOnStartupEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_vrControllersDisconnectedOnStartupEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "add_vrControllersDisconnectedOnStartupEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_vrFocusWasCapturedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_vrFocusWasCapturedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "add_vrFocusWasCapturedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_vrFocusWasReleasedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_vrFocusWasReleasedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "add_vrFocusWasReleasedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasInputFocus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_hasInputFocus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "get_hasInputFocus", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasVrFocus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_hasVrFocus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "get_hasVrFocus", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isAlwaysWireless(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isAlwaysWireless")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "get_isAlwaysWireless", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_vrPlatformSDK(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::VRPlatformSDK> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::VRPlatformSDK,
                0usize,
            >("get_vrPlatformSDK")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "get_vrPlatformSDK", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::VRPlatformSDK = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_controllersDidChangeReferenceEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_controllersDidChangeReferenceEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "remove_controllersDidChangeReferenceEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_controllersDidDisconnectEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_controllersDidDisconnectEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "remove_controllersDidDisconnectEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_hmdMountedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_hmdMountedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "remove_hmdMountedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_hmdUnmountedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_hmdUnmountedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "remove_hmdUnmountedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_inputFocusWasCapturedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_inputFocusWasCapturedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "remove_inputFocusWasCapturedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_inputFocusWasReleasedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_inputFocusWasReleasedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "remove_inputFocusWasReleasedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_vrControllersDisconnectedOnStartupEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_vrControllersDisconnectedOnStartupEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "remove_vrControllersDisconnectedOnStartupEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_vrFocusWasCapturedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_vrFocusWasCapturedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "remove_vrFocusWasCapturedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_vrFocusWasReleasedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_vrFocusWasReleasedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper as quest_hook::libil2cpp::Type >
                    ::class(), "remove_vrFocusWasReleasedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
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
impl AsRef<crate::GlobalNamespace::IVRPlatformHelper>
for crate::GlobalNamespace::PSVR2Helper {
    fn as_ref(&self) -> &crate::GlobalNamespace::IVRPlatformHelper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PSVR2Helper")]
impl AsMut<crate::GlobalNamespace::IVRPlatformHelper>
for crate::GlobalNamespace::PSVR2Helper {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IVRPlatformHelper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PSVR2Helper_HeadsetHapticFrequencyLimit {
    pub maxTimePlayed: f32,
    pub maximumFrequency: i32,
}
#[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PSVR2Helper/HeadsetHapticFrequencyLimit";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "PSVR2Helper+HeadsetHapticFrequencyLimit")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32, i32), quest_hook::libil2cpp::Void, 2usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PSVR2Helper_HeadsetHapticFrequencyLimit as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (maxTimePlayed, maximumFrequency))?
        };
        Ok(__cordl_ret.into())
    }
}
