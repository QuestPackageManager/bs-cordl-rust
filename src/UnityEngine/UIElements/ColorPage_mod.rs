#[cfg(feature = "UnityEngine+UIElements+ColorPage")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ColorPage {
    pub isValid: bool,
    pub pageAndID: crate::UnityEngine::Color32,
}
#[cfg(feature = "UnityEngine+UIElements+ColorPage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ColorPage =>
    "UnityEngine.UIElements"."ColorPage"
);
#[cfg(feature = "UnityEngine+UIElements+ColorPage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::ColorPage {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ColorPage")]
impl crate::UnityEngine::UIElements::ColorPage {
    pub fn ToNativeColorPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToNativeColorPage",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
