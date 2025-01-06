#[cfg(feature = "UnityEngine+UIVertex")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UIVertex {
    pub position: crate::UnityEngine::Vector3,
    pub normal: crate::UnityEngine::Vector3,
    pub tangent: crate::UnityEngine::Vector4,
    pub color: crate::UnityEngine::Color32,
    pub uv0: crate::UnityEngine::Vector4,
    pub uv1: crate::UnityEngine::Vector4,
    pub uv2: crate::UnityEngine::Vector4,
    pub uv3: crate::UnityEngine::Vector4,
}
#[cfg(feature = "UnityEngine+UIVertex")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIVertex => "UnityEngine"
    ."UIVertex"
);
#[cfg(feature = "UnityEngine+UIVertex")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::UIVertex {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIVertex")]
impl crate::UnityEngine::UIVertex {}
