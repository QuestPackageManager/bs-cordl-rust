#[cfg(feature = "UnityEngine+Rendering+BatchDrawRange")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BatchDrawRange {
    pub drawCommandsBegin: u32,
    pub drawCommandsCount: u32,
    pub filterSettings: crate::UnityEngine::Rendering::BatchFilterSettings,
}
#[cfg(feature = "UnityEngine+Rendering+BatchDrawRange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchDrawRange =>
    "UnityEngine.Rendering"."BatchDrawRange"
);
#[cfg(feature = "UnityEngine+Rendering+BatchDrawRange")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchDrawRange {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchDrawRange")]
impl crate::UnityEngine::Rendering::BatchDrawRange {}
