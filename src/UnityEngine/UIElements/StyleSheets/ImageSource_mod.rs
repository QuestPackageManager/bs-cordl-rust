#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ImageSource")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageSource {
    pub texture: *mut crate::UnityEngine::Texture2D,
    pub sprite: *mut crate::UnityEngine::Sprite,
    pub vectorImage: *mut crate::UnityEngine::UIElements::VectorImage,
    pub renderTexture: *mut crate::UnityEngine::RenderTexture,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ImageSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::ImageSource =>
    "UnityEngine.UIElements.StyleSheets"."ImageSource"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ImageSource")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::ImageSource {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ImageSource")]
impl crate::UnityEngine::UIElements::StyleSheets::ImageSource {
    pub fn IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNull",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
