#[cfg(feature = "UnityEngine+AnimatorTransitionInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AnimatorTransitionInfo {
    pub m_FullPath: i32,
    pub m_UserName: i32,
    pub m_Name: i32,
    pub m_HasFixedDuration: bool,
    pub m_Duration: f32,
    pub m_NormalizedTime: f32,
    pub m_AnyState: bool,
    pub m_TransitionType: i32,
}
#[cfg(feature = "UnityEngine+AnimatorTransitionInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimatorTransitionInfo =>
    "UnityEngine"."AnimatorTransitionInfo"
);
#[cfg(feature = "UnityEngine+AnimatorTransitionInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::AnimatorTransitionInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+AnimatorTransitionInfo")]
impl crate::UnityEngine::AnimatorTransitionInfo {}
