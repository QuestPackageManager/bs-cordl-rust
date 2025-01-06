#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceResetEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeviceResetEvent {
    padding: quest_hook::libil2cpp::ValueTypePadding<9usize>,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceResetEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::DeviceResetEvent =>
    "UnityEngine.InputSystem.LowLevel"."DeviceResetEvent"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceResetEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::DeviceResetEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceResetEvent")]
impl crate::UnityEngine::InputSystem::LowLevel::DeviceResetEvent {
    pub const Type: i32 = 1146245972i32;
    pub fn Create(
        deviceId: i32,
        hardReset: bool,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::DeviceResetEvent,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::DeviceResetEvent = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (deviceId, hardReset, _cordl_time))?;
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceResetEvent")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::DeviceResetEvent {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceResetEvent")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::DeviceResetEvent {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo {
        todo!()
    }
}
