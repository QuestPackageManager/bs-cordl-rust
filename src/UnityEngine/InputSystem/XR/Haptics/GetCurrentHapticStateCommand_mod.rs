#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+GetCurrentHapticStateCommand")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GetCurrentHapticStateCommand {
    padding: [u8; 16usize],
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+GetCurrentHapticStateCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::Haptics::GetCurrentHapticStateCommand =>
    "UnityEngine.InputSystem.XR.Haptics"."GetCurrentHapticStateCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+GetCurrentHapticStateCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::Haptics::GetCurrentHapticStateCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+GetCurrentHapticStateCommand")]
impl crate::UnityEngine::InputSystem::XR::Haptics::GetCurrentHapticStateCommand {
    pub const kSize: i32 = 16i32;
    pub fn Create() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::XR::Haptics::GetCurrentHapticStateCommand,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::XR::Haptics::GetCurrentHapticStateCommand = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::XR::Haptics::HapticState,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::XR::Haptics::HapticState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_currentState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_typeStatic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_typeStatic",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+GetCurrentHapticStateCommand")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
for crate::UnityEngine::InputSystem::XR::Haptics::GetCurrentHapticStateCommand {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+GetCurrentHapticStateCommand")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
for crate::UnityEngine::InputSystem::XR::Haptics::GetCurrentHapticStateCommand {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
