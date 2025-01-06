#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BufferedRumble {
    pub _capabilities_k__BackingField: crate::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities,
    pub _device_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputDevice,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::Haptics::BufferedRumble =>
    "UnityEngine.InputSystem.XR.Haptics"."BufferedRumble"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::Haptics::BufferedRumble {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
impl crate::UnityEngine::InputSystem::XR::Haptics::BufferedRumble {
    pub fn EnqueueRumble(
        &mut self,
        samples: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "EnqueueRumble",
            (samples),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (device),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_capabilities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_capabilities",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_device(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_device", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_capabilities(
        &mut self,
        value: crate::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_capabilities",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_device(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_device",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
