#[cfg(feature = "UnityEngine+UICharInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct UICharInfo {
    pub cursorPos: crate::UnityEngine::Vector2,
    pub charWidth: f32,
}
#[cfg(feature = "UnityEngine+UICharInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UICharInfo => "UnityEngine"
    ."UICharInfo"
);
#[cfg(feature = "UnityEngine+UICharInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::UICharInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UICharInfo")]
impl crate::UnityEngine::UICharInfo {}
