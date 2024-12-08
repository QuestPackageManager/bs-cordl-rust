#[cfg(feature = "UnityEngine+RenderInstancedDataLayout")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RenderInstancedDataLayout {
    pub _size_k__BackingField: i32,
    pub _offsetObjectToWorld_k__BackingField: i32,
    pub _offsetPrevObjectToWorld_k__BackingField: i32,
    pub _offsetRenderingLayerMask_k__BackingField: i32,
}
#[cfg(feature = "UnityEngine+RenderInstancedDataLayout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderInstancedDataLayout =>
    "UnityEngine"."RenderInstancedDataLayout"
);
#[cfg(feature = "UnityEngine+RenderInstancedDataLayout")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::RenderInstancedDataLayout {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+RenderInstancedDataLayout")]
impl crate::UnityEngine::RenderInstancedDataLayout {}
