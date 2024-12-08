#[cfg(feature = "UnityEngine+TextCore+Text+TextVertex")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextVertex {
    pub position: crate::UnityEngine::Vector3,
    pub uv: crate::UnityEngine::Vector4,
    pub uv2: crate::UnityEngine::Vector2,
    pub color: crate::UnityEngine::Color32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextVertex")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextVertex =>
    "UnityEngine.TextCore.Text"."TextVertex"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextVertex")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::TextVertex {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextVertex")]
impl crate::UnityEngine::TextCore::Text::TextVertex {}
