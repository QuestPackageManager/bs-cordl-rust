#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DeviceRemoveEvent {
    padding: [u8; 20usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent =>
    "UnityEngine.InputSystem.LowLevel"."DeviceRemoveEvent"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
impl crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
    pub const Type: i32 = 1146242381i32;
    pub fn ToEventPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToEventPtr",
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
