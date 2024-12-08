#[cfg(feature = "UnityEngine+UILineInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UILineInfo {
    pub startCharIdx: i32,
    pub height: i32,
    pub topY: f32,
    pub leading: f32,
}
#[cfg(feature = "UnityEngine+UILineInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UILineInfo => "UnityEngine"
    ."UILineInfo"
);
#[cfg(feature = "UnityEngine+UILineInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::UILineInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UILineInfo")]
impl crate::UnityEngine::UILineInfo {}
