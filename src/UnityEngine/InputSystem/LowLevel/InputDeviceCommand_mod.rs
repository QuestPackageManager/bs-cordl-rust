#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputDeviceCommand")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputDeviceCommand {
    padding: [u8; 8usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputDeviceCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputDeviceCommand =>
    "UnityEngine.InputSystem.LowLevel"."InputDeviceCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputDeviceCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputDeviceCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputDeviceCommand")]
impl crate::UnityEngine::InputSystem::LowLevel::InputDeviceCommand {
    pub const BaseCommandSize: i32 = 8i32;
    pub const GenericFailure: i64 = -1i64;
    pub const GenericSuccess: i64 = 1i64;
    pub const kBaseCommandSize: i32 = 8i32;
    pub fn get_payloadPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_payloadPtr",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_payloadSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_payloadSizeInBytes",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::UnityEngine::InputSystem::Utilities::FourCC,
        sizeInBytes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type, sizeInBytes),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
