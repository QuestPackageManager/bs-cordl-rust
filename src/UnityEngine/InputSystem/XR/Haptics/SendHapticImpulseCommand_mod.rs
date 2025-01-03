#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendHapticImpulseCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SendHapticImpulseCommand {
    padding: [u8; 20usize],
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendHapticImpulseCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::Haptics::SendHapticImpulseCommand =>
    "UnityEngine.InputSystem.XR.Haptics"."SendHapticImpulseCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendHapticImpulseCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::Haptics::SendHapticImpulseCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendHapticImpulseCommand")]
impl crate::UnityEngine::InputSystem::XR::Haptics::SendHapticImpulseCommand {
    pub const kSize: i32 = 20i32;
    pub fn Create(
        motorChannel: i32,
        motorAmplitude: f32,
        motorDuration: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::XR::Haptics::SendHapticImpulseCommand,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::XR::Haptics::SendHapticImpulseCommand = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (motorChannel, motorAmplitude, motorDuration))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Type", ())?;
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
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendHapticImpulseCommand")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
for crate::UnityEngine::InputSystem::XR::Haptics::SendHapticImpulseCommand {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendHapticImpulseCommand")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
for crate::UnityEngine::InputSystem::XR::Haptics::SendHapticImpulseCommand {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
