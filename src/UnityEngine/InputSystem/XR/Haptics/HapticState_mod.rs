#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+HapticState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HapticState {
    pub _samplesQueued_k__BackingField: u32,
    pub _samplesAvailable_k__BackingField: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+HapticState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::Haptics::HapticState =>
    "UnityEngine.InputSystem.XR.Haptics"."HapticState"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+HapticState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::Haptics::HapticState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+HapticState")]
impl crate::UnityEngine::InputSystem::XR::Haptics::HapticState {
    pub fn _ctor(
        &mut self,
        samplesQueued: u32,
        samplesAvailable: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (samplesQueued, samplesAvailable),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_samplesAvailable(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_samplesAvailable",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_samplesQueued(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_samplesQueued",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_samplesAvailable(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_samplesAvailable",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_samplesQueued(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_samplesQueued",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
