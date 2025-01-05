#[cfg(feature = "UnityEngine+XR+InputDevice")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputDevice {
    pub m_DeviceId: u64,
    pub m_Initialized: bool,
}
#[cfg(feature = "UnityEngine+XR+InputDevice")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::InputDevice => "UnityEngine.XR"
    ."InputDevice"
);
#[cfg(feature = "UnityEngine+XR+InputDevice")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::XR::InputDevice {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+InputDevice")]
impl crate::UnityEngine::XR::InputDevice {
    pub fn Equals_Gc0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_InputDevice1(
        &mut self,
        other: crate::UnityEngine::XR::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidId(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValidId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendHapticImpulse(
        &mut self,
        channel: u32,
        amplitude: f32,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SendHapticImpulse",
            (channel, amplitude, duration),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn StopHaptics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StopHaptics",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        deviceId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (deviceId),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceId(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deviceId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_manufacturer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_manufacturer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+InputDevice")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::InputDevice>>
for crate::UnityEngine::XR::InputDevice {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::InputDevice> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+XR+InputDevice")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::InputDevice>>
for crate::UnityEngine::XR::InputDevice {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::InputDevice> {
        todo!()
    }
}
