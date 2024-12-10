#[cfg(feature = "UnityEngine+UIElements+StyleSheets+SelectorMatchRecord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SelectorMatchRecord {
    pub sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
    pub styleSheetIndexInStack: i32,
    pub complexSelector: *mut crate::UnityEngine::UIElements::StyleComplexSelector,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+SelectorMatchRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord =>
    "UnityEngine.UIElements.StyleSheets"."SelectorMatchRecord"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+SelectorMatchRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+SelectorMatchRecord")]
impl crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord {
    pub fn _ctor(
        &mut self,
        sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
        styleSheetIndexInStack: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (sheet, styleSheetIndexInStack),
        )?;
        Ok(__cordl_ret.into())
    }
}
