#[cfg(feature = "UnityEngine+UIElements+UIR+State")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct State {
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub texture: crate::UnityEngine::UIElements::TextureId,
    pub stencilRef: i32,
    pub sdfScale: f32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::State =>
    "UnityEngine.UIElements.UIR"."State"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+State")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::State {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+State")]
impl crate::UnityEngine::UIElements::UIR::State {}
