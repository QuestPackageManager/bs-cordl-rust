#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+HapticCapabilities")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HapticCapabilities {
    pub _numChannels_k__BackingField: u32,
    pub _frequencyHz_k__BackingField: u32,
    pub _maxBufferSize_k__BackingField: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+HapticCapabilities")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities =>
    "UnityEngine.InputSystem.XR.Haptics"."HapticCapabilities"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+HapticCapabilities")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+HapticCapabilities")]
impl crate::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities {
    pub fn _ctor(
        &mut self,
        numChannels: u32,
        frequencyHz: u32,
        maxBufferSize: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (numChannels, frequencyHz, maxBufferSize),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_frequencyHz(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_frequencyHz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxBufferSize(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maxBufferSize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_numChannels(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_numChannels",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_frequencyHz(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_frequencyHz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxBufferSize(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_maxBufferSize",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_numChannels(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_numChannels",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
