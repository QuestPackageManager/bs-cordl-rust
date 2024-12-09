#[cfg(feature = "UnityEngine+UIElements+StyleSheets+MatchResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub errorCode: crate::UnityEngine::UIElements::StyleSheets::MatchResultErrorCode,
    pub errorValue: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+MatchResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::MatchResult =>
    "UnityEngine.UIElements.StyleSheets"."MatchResult"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+MatchResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::MatchResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+MatchResult")]
impl crate::UnityEngine::UIElements::StyleSheets::MatchResult {
    pub fn get_success(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_success",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
