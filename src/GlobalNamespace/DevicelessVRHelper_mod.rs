#[cfg(feature = "DevicelessVRHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct DevicelessVRHelper {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub inputFocusWasCapturedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub inputFocusWasReleasedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub vrFocusWasCapturedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub vrFocusWasReleasedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub hmdUnmountedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub hmdMountedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub controllersDidChangeReferenceEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub controllersDidDisconnectEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _hasInputFocus: bool,
    pub _hasVrFocus: bool,
    pub _scrollingLastFrame: bool,
}
#[cfg(feature = "DevicelessVRHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DevicelessVRHelper => ""
    ."DevicelessVRHelper"
);
#[cfg(feature = "DevicelessVRHelper")]
impl std::ops::Deref for crate::GlobalNamespace::DevicelessVRHelper {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DevicelessVRHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::DevicelessVRHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DevicelessVRHelper")]
impl crate::GlobalNamespace::DevicelessVRHelper {
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
    pub fn GetThumbstickValue(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetThumbstickValue", (node))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_loggerPrefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_loggerPrefix", ())?;
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
#[cfg(feature = "DevicelessVRHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::DevicelessVRHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DevicelessVRHelper")]
impl AsRef<crate::GlobalNamespace::IVRPlatformHelper>
for crate::GlobalNamespace::DevicelessVRHelper {
    fn as_ref(&self) -> &crate::GlobalNamespace::IVRPlatformHelper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DevicelessVRHelper")]
impl AsMut<crate::GlobalNamespace::IVRPlatformHelper>
for crate::GlobalNamespace::DevicelessVRHelper {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IVRPlatformHelper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DevicelessVRHelper")]
impl AsRef<crate::GlobalNamespace::IVerboseLogger>
for crate::GlobalNamespace::DevicelessVRHelper {
    fn as_ref(&self) -> &crate::GlobalNamespace::IVerboseLogger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DevicelessVRHelper")]
impl AsMut<crate::GlobalNamespace::IVerboseLogger>
for crate::GlobalNamespace::DevicelessVRHelper {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IVerboseLogger {
        unsafe { std::mem::transmute(self) }
    }
}
