#[cfg(feature = "UnityEngine+UIElements+UIR+DrawBufferRange")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DrawBufferRange {
    pub firstIndex: i32,
    pub indexCount: i32,
    pub minIndexVal: i32,
    pub vertsReferenced: i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+DrawBufferRange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::DrawBufferRange =>
    "UnityEngine.UIElements.UIR"."DrawBufferRange"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+DrawBufferRange")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::DrawBufferRange {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+DrawBufferRange")]
impl crate::UnityEngine::UIElements::UIR::DrawBufferRange {}
