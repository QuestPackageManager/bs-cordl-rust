#[cfg(feature = "UnityEngine+UIElements+GradientSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GradientSettings {
    pub gradientType: crate::UnityEngine::UIElements::GradientType,
    pub addressMode: crate::UnityEngine::UIElements::AddressMode,
    pub radialFocus: crate::UnityEngine::Vector2,
    pub location: crate::UnityEngine::RectInt,
}
#[cfg(feature = "UnityEngine+UIElements+GradientSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::GradientSettings =>
    "UnityEngine.UIElements"."GradientSettings"
);
#[cfg(feature = "UnityEngine+UIElements+GradientSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::GradientSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GradientSettings")]
impl crate::UnityEngine::UIElements::GradientSettings {}
