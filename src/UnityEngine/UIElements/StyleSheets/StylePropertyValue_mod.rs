#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyValue")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StylePropertyValue {
    pub sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
    pub handle: crate::UnityEngine::UIElements::StyleValueHandle,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyValue")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StylePropertyValue =>
    "UnityEngine.UIElements.StyleSheets"."StylePropertyValue"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyValue")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyValue")]
impl crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue {}
