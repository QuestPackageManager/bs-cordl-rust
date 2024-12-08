#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionProperty")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ComputedTransitionProperty {
    pub id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    pub durationMs: i32,
    pub delayMs: i32,
    pub easingCurve: *mut crate::System::Func_2<f32, f32>,
}
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionProperty")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ComputedTransitionProperty => "UnityEngine.UIElements"
    ."ComputedTransitionProperty"
);
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionProperty")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::ComputedTransitionProperty {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionProperty")]
impl crate::UnityEngine::UIElements::ComputedTransitionProperty {}
