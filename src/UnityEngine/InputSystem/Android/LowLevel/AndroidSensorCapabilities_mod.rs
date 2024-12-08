#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorCapabilities")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AndroidSensorCapabilities {
    pub sensorType: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorType,
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorCapabilities")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorCapabilities =>
    "UnityEngine.InputSystem.Android.LowLevel"."AndroidSensorCapabilities"
);
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorCapabilities")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorCapabilities {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorCapabilities")]
impl crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorCapabilities {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToJson(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToJson",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
