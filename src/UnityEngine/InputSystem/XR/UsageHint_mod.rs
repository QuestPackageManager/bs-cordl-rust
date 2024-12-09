#[cfg(feature = "UnityEngine+InputSystem+XR+UsageHint")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UsageHint {
    pub content: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+UsageHint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::UsageHint =>
    "UnityEngine.InputSystem.XR"."UsageHint"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+UsageHint")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::UsageHint {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+UsageHint")]
impl crate::UnityEngine::InputSystem::XR::UsageHint {}
