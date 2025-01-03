#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleValueManaged")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StyleValueManaged {
    pub id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    pub keyword: crate::UnityEngine::UIElements::StyleKeyword,
    pub value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleValueManaged")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StyleValueManaged =>
    "UnityEngine.UIElements.StyleSheets"."StyleValueManaged"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleValueManaged")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleValueManaged")]
impl crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged {}
