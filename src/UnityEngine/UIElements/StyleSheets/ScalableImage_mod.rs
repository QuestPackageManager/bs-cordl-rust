#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ScalableImage")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ScalableImage {
    pub normalImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub highResolutionImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ScalableImage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::ScalableImage =>
    "UnityEngine.UIElements.StyleSheets"."ScalableImage"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ScalableImage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::ScalableImage {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ScalableImage")]
impl crate::UnityEngine::UIElements::StyleSheets::ScalableImage {
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
