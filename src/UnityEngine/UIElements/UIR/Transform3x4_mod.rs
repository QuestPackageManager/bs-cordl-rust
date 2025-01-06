#[cfg(feature = "UnityEngine+UIElements+UIR+Transform3x4")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Transform3x4 {
    pub v0: crate::UnityEngine::Vector4,
    pub v1: crate::UnityEngine::Vector4,
    pub v2: crate::UnityEngine::Vector4,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Transform3x4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::Transform3x4 =>
    "UnityEngine.UIElements.UIR"."Transform3x4"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Transform3x4")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::Transform3x4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Transform3x4")]
impl crate::UnityEngine::UIElements::UIR::Transform3x4 {}
