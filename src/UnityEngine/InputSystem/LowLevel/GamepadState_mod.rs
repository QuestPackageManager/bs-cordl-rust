#[cfg(feature = "UnityEngine+InputSystem+LowLevel+GamepadState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GamepadState {
    padding: [u8; 28usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+GamepadState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::GamepadState
    => "UnityEngine.InputSystem.LowLevel"."GamepadState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+GamepadState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::GamepadState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+GamepadState")]
impl crate::UnityEngine::InputSystem::LowLevel::GamepadState {
    pub const ButtonEastShortDisplayName: &'static str = "B";
    pub const ButtonNorthShortDisplayName: &'static str = "Y";
    pub const ButtonSouthShortDisplayName: &'static str = "A";
    pub const ButtonWestShortDisplayName: &'static str = "X";
    pub fn WithButton(
        &mut self,
        button: crate::UnityEngine::InputSystem::LowLevel::GamepadButton,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::GamepadState,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::GamepadState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithButton",
            (button, value),
        )?;
        Ok(__cordl_ret)
    }
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
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        buttons: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::LowLevel::GamepadButton,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (buttons),
        )?;
        Ok(__cordl_ret)
    }
}
