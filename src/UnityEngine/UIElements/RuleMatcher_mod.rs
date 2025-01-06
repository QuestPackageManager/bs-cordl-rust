#[cfg(feature = "UnityEngine+UIElements+RuleMatcher")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RuleMatcher {
    pub sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
    pub complexSelector: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleComplexSelector,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RuleMatcher")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RuleMatcher =>
    "UnityEngine.UIElements"."RuleMatcher"
);
#[cfg(feature = "UnityEngine+UIElements+RuleMatcher")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::RuleMatcher {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RuleMatcher")]
impl crate::UnityEngine::UIElements::RuleMatcher {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
