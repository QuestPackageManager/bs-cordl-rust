#[cfg(feature = "UnityEngine+UIElements+VectorImageVertex")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VectorImageVertex {
    pub position: crate::UnityEngine::Vector3,
    pub tint: crate::UnityEngine::Color32,
    pub uv: crate::UnityEngine::Vector2,
    pub settingIndex: u32,
    pub flags: crate::UnityEngine::Color32,
    pub circle: crate::UnityEngine::Vector4,
}
#[cfg(feature = "UnityEngine+UIElements+VectorImageVertex")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VectorImageVertex =>
    "UnityEngine.UIElements"."VectorImageVertex"
);
#[cfg(feature = "UnityEngine+UIElements+VectorImageVertex")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VectorImageVertex {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VectorImageVertex")]
impl crate::UnityEngine::UIElements::VectorImageVertex {}
