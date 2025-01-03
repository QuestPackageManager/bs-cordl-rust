#[cfg(feature = "UnityEngine+AnimatorClipInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct AnimatorClipInfo {
    pub m_ClipInstanceID: i32,
    pub m_Weight: f32,
}
#[cfg(feature = "UnityEngine+AnimatorClipInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimatorClipInfo => "UnityEngine"
    ."AnimatorClipInfo"
);
#[cfg(feature = "UnityEngine+AnimatorClipInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::AnimatorClipInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+AnimatorClipInfo")]
impl crate::UnityEngine::AnimatorClipInfo {}
