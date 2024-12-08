#[cfg(feature = "UnityEngine+UIElements+UIR+TextureEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextureEntry {
    pub source: *mut crate::UnityEngine::Texture,
    pub actual: crate::UnityEngine::UIElements::TextureId,
    pub replaced: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::TextureEntry =>
    "UnityEngine.UIElements.UIR"."TextureEntry"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::TextureEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureEntry")]
impl crate::UnityEngine::UIElements::UIR::TextureEntry {}
