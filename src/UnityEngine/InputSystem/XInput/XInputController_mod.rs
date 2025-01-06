#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController")]
#[repr(C)]
#[derive(Debug)]
pub struct XInputController {
    __cordl_parent: crate::UnityEngine::InputSystem::Gamepad,
    pub _menu_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::ButtonControl,
    >,
    pub _view_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::ButtonControl,
    >,
    pub m_HaveParsedCapabilities: bool,
    pub m_SubType: crate::UnityEngine::InputSystem::XInput::XInputController_DeviceSubType,
    pub m_Flags: crate::UnityEngine::InputSystem::XInput::XInputController_DeviceFlags,
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XInput::XInputController =>
    "UnityEngine.InputSystem.XInput"."XInputController"
);
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XInput::XInputController {
    type Target = crate::UnityEngine::InputSystem::Gamepad;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::XInput::XInputController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController")]
impl crate::UnityEngine::InputSystem::XInput::XInputController {
    #[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+Capabilities")]
    pub type Capabilities = crate::UnityEngine::InputSystem::XInput::XInputController_Capabilities;
    #[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+DeviceFlags")]
    pub type DeviceFlags = crate::UnityEngine::InputSystem::XInput::XInputController_DeviceFlags;
    #[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+DeviceSubType")]
    pub type DeviceSubType = crate::UnityEngine::InputSystem::XInput::XInputController_DeviceSubType;
    #[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+DeviceType")]
    pub type DeviceType = crate::UnityEngine::InputSystem::XInput::XInputController_DeviceType;
    pub fn FinishSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishSetup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ParseCapabilities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseCapabilities", ())?;
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
    pub fn get_flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::XInput::XInputController_DeviceFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::XInput::XInputController_DeviceFlags = __cordl_object
            .invoke("get_flags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_menu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object.invoke("get_menu", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::XInput::XInputController_DeviceSubType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::XInput::XInputController_DeviceSubType = __cordl_object
            .invoke("get_subType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_view(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object.invoke("get_view", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_menu(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_menu", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_view(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_view", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::XInput::XInputController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+Capabilities")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XInputController_Capabilities {
    pub _cordl_type: crate::UnityEngine::InputSystem::XInput::XInputController_DeviceType,
    pub subType: crate::UnityEngine::InputSystem::XInput::XInputController_DeviceSubType,
    pub flags: crate::UnityEngine::InputSystem::XInput::XInputController_DeviceFlags,
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+Capabilities")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XInput::XInputController_Capabilities =>
    "UnityEngine.InputSystem.XInput"."XInputController/Capabilities"
);
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+Capabilities")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XInput::XInputController_Capabilities {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+Capabilities")]
impl crate::UnityEngine::InputSystem::XInput::XInputController_Capabilities {}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+DeviceFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XInputController_DeviceFlags {
    #[default]
    ForceFeedbackSupported = 1i32,
    NoNavigation = 16i32,
    PluginModulesSupported = 8i32,
    VoiceSupported = 4i32,
    Wireless = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+DeviceFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XInput::XInputController_DeviceFlags =>
    "UnityEngine.InputSystem.XInput"."XInputController/DeviceFlags"
);
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+DeviceSubType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XInputController_DeviceSubType {
    #[default]
    ArcadePad = 19i32,
    ArcadeStick = 3i32,
    DancePad = 5i32,
    DrumKit = 8i32,
    FlightStick = 4i32,
    Gamepad = 1i32,
    Guitar = 6i32,
    GuitarAlternate = 7i32,
    GuitarBass = 11i32,
    Unknown = 0i32,
    Wheel = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+DeviceSubType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XInput::XInputController_DeviceSubType =>
    "UnityEngine.InputSystem.XInput"."XInputController/DeviceSubType"
);
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+DeviceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XInputController_DeviceType {
    #[default]
    Gamepad = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputController+DeviceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XInput::XInputController_DeviceType =>
    "UnityEngine.InputSystem.XInput"."XInputController/DeviceType"
);
