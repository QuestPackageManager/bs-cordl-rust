#[cfg(feature = "UnityEngine+InputSystem+LowLevel+JoystickState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct JoystickState {
    pub buttons: i32,
    pub stick: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+JoystickState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::JoystickState =>
    "UnityEngine.InputSystem.LowLevel"."JoystickState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+JoystickState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::JoystickState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+JoystickState")]
impl crate::UnityEngine::InputSystem::LowLevel::JoystickState {
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+JoystickState+Button")]
    pub type Button = crate::UnityEngine::InputSystem::LowLevel::JoystickState_Button;
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_format",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_kFormat() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_kFormat", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+JoystickState")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::JoystickState {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+JoystickState")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::JoystickState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+JoystickState+Button")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoystickState_Button {
    HatSwitchDown = 1i32,
    HatSwitchLeft = 2i32,
    HatSwitchRight = 3i32,
    HatSwitchUp = 0i32,
    Trigger = 4i32,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+JoystickState+Button")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::JoystickState_Button =>
    "UnityEngine.InputSystem.LowLevel"."JoystickState/Button"
);
