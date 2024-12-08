#[cfg(feature = "TMPro+TMP_Vertex")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_Vertex {
    pub position: crate::UnityEngine::Vector3,
    pub uv: crate::UnityEngine::Vector2,
    pub uv2: crate::UnityEngine::Vector2,
    pub uv4: crate::UnityEngine::Vector2,
    pub color: crate::UnityEngine::Color32,
}
#[cfg(feature = "TMPro+TMP_Vertex")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Vertex => "TMPro"."TMP_Vertex"
);
#[cfg(feature = "TMPro+TMP_Vertex")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_Vertex {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_Vertex")]
impl crate::TMPro::TMP_Vertex {}
