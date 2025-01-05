#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleValue")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StyleValue {
    padding: quest_hook::libil2cpp::ValueTypePadding<24usize>,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleValue")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleSheets::StyleValue
    => "UnityEngine.UIElements.StyleSheets"."StyleValue"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleValue")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::StyleValue {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleValue")]
impl crate::UnityEngine::UIElements::StyleSheets::StyleValue {}
