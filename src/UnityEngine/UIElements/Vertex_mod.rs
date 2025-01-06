#[cfg(feature = "UnityEngine+UIElements+Vertex")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vertex {
    pub position: crate::UnityEngine::Vector3,
    pub tint: crate::UnityEngine::Color32,
    pub uv: crate::UnityEngine::Vector2,
    pub xformClipPages: crate::UnityEngine::Color32,
    pub ids: crate::UnityEngine::Color32,
    pub flags: crate::UnityEngine::Color32,
    pub opacityColorPages: crate::UnityEngine::Color32,
    pub settingIndex: crate::UnityEngine::Color32,
    pub circle: crate::UnityEngine::Vector4,
    pub textureId: f32,
}
#[cfg(feature = "UnityEngine+UIElements+Vertex")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Vertex =>
    "UnityEngine.UIElements"."Vertex"
);
#[cfg(feature = "UnityEngine+UIElements+Vertex")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Vertex {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vertex")]
impl crate::UnityEngine::UIElements::Vertex {}
