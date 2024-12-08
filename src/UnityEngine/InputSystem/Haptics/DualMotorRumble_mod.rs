#[cfg(feature = "UnityEngine+InputSystem+Haptics+DualMotorRumble")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DualMotorRumble {
    pub _lowFrequencyMotorSpeed_k__BackingField: f32,
    pub _highFrequencyMotorSpeed_k__BackingField: f32,
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+DualMotorRumble")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Haptics::DualMotorRumble =>
    "UnityEngine.InputSystem.Haptics"."DualMotorRumble"
);
#[cfg(feature = "UnityEngine+InputSystem+Haptics+DualMotorRumble")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Haptics::DualMotorRumble {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+DualMotorRumble")]
impl crate::UnityEngine::InputSystem::Haptics::DualMotorRumble {
    pub fn PauseHaptics(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PauseHaptics",
            (device),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ResetHaptics(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ResetHaptics",
            (device),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ResumeHaptics(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ResumeHaptics",
            (device),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetMotorSpeeds(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        lowFrequency: f32,
        highFrequency: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetMotorSpeeds",
            (device, lowFrequency, highFrequency),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_highFrequencyMotorSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_highFrequencyMotorSpeed",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isRumbling(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isRumbling",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_lowFrequencyMotorSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_lowFrequencyMotorSpeed",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_highFrequencyMotorSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_highFrequencyMotorSpeed",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_lowFrequencyMotorSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_lowFrequencyMotorSpeed",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
