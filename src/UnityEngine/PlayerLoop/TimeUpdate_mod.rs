#[cfg(feature = "UnityEngine+PlayerLoop+TimeUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TimeUpdate {}
#[cfg(feature = "UnityEngine+PlayerLoop+TimeUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerLoop::TimeUpdate =>
    "UnityEngine.PlayerLoop"."TimeUpdate"
);
#[cfg(feature = "UnityEngine+PlayerLoop+TimeUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::TimeUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+TimeUpdate")]
impl crate::UnityEngine::PlayerLoop::TimeUpdate {
    #[cfg(
        feature = "UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
    )]
    pub type WaitForLastPresentationAndUpdateTime = crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime;
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TimeUpdate_WaitForLastPresentationAndUpdateTime {}
#[cfg(
    feature = "UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime =>
    "UnityEngine.PlayerLoop"."TimeUpdate/WaitForLastPresentationAndUpdateTime"
);
#[cfg(
    feature = "UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
impl crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime {}
