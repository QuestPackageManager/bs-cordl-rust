#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+WMRSpatialController")]
#[repr(C)]
#[derive(Debug)]
pub struct WMRSpatialController {
    __cordl_parent: crate::UnityEngine::InputSystem::XR::XRControllerWithRumble,
    pub _joystick_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
    pub _touchpad_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
    pub _grip_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    pub _gripPressed_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub _menu_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub _trigger_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    pub _triggerPressed_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub _joystickClicked_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub _touchpadClicked_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub _touchpadTouched_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub _deviceVelocity_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    pub _deviceAngularVelocity_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    pub _batteryLevel_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    pub _sourceLossRisk_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    pub _sourceLossMitigationDirection_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    pub _pointerPosition_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    pub _pointerRotation_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl,
}
#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+WMRSpatialController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::XR::WindowsMR::Input::WMRSpatialController =>
    "UnityEngine.XR.WindowsMR.Input"."WMRSpatialController"
);
#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+WMRSpatialController")]
impl std::ops::Deref for crate::UnityEngine::XR::WindowsMR::Input::WMRSpatialController {
    type Target = crate::UnityEngine::InputSystem::XR::XRControllerWithRumble;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+WMRSpatialController")]
impl std::ops::DerefMut
for crate::UnityEngine::XR::WindowsMR::Input::WMRSpatialController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+WMRSpatialController")]
impl crate::UnityEngine::XR::WindowsMR::Input::WMRSpatialController {
    pub fn get_batteryLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("get_batteryLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_touchpadTouched(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("get_touchpadTouched", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_grip(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_grip", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_touchpad(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_touchpad", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_pointerRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl = __cordl_object
            .invoke("get_pointerRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_batteryLevel(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_batteryLevel", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_menu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("get_menu", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_triggerPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("get_triggerPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_trigger(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trigger", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_pointerRotation(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pointerRotation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_sourceLossRisk(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("get_sourceLossRisk", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_pointerPosition(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pointerPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_joystick(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_joystick", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_sourceLossMitigationDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control = __cordl_object
            .invoke("get_sourceLossMitigationDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_joystick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control = __cordl_object
            .invoke("get_joystick", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trigger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("get_trigger", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pointerPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control = __cordl_object
            .invoke("get_pointerPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_gripPressed(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gripPressed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_gripPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("get_gripPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_touchpadClicked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("get_touchpadClicked", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_grip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::AxisControl = __cordl_object
            .invoke("get_grip", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_deviceVelocity(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deviceVelocity", (value))?;
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
    pub fn set_joystickClicked(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_joystickClicked", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sourceLossMitigationDirection(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sourceLossMitigationDirection", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_joystickClicked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("get_joystickClicked", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_menu(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_menu", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_deviceVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control = __cordl_object
            .invoke("get_deviceVelocity", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_triggerPressed(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_triggerPressed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sourceLossRisk(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::AxisControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sourceLossRisk", (value))?;
        Ok(__cordl_ret)
    }
    pub fn FinishSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishSetup", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_touchpadTouched(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_touchpadTouched", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_touchpadClicked(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_touchpadClicked", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_touchpad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control = __cordl_object
            .invoke("get_touchpad", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_deviceAngularVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control = __cordl_object
            .invoke("get_deviceAngularVelocity", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_deviceAngularVelocity(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deviceAngularVelocity", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+WMRSpatialController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::WindowsMR::Input::WMRSpatialController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
