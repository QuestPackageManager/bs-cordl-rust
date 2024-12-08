#[cfg(feature = "UnityEngine+UIElements+StyleSheets+MatchResultInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MatchResultInfo {
    pub success: bool,
    pub triggerPseudoMask: crate::UnityEngine::UIElements::PseudoStates,
    pub dependencyPseudoMask: crate::UnityEngine::UIElements::PseudoStates,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+MatchResultInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::MatchResultInfo =>
    "UnityEngine.UIElements.StyleSheets"."MatchResultInfo"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+MatchResultInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+MatchResultInfo")]
impl crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo {
    pub fn _ctor(
        &mut self,
        success: bool,
        triggerPseudoMask: crate::UnityEngine::UIElements::PseudoStates,
        dependencyPseudoMask: crate::UnityEngine::UIElements::PseudoStates,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (success, triggerPseudoMask, dependencyPseudoMask),
        )?;
        Ok(__cordl_ret)
    }
}
