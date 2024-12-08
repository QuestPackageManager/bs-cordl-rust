#[cfg(feature = "UnityEngine+UIElements+PanelClearSettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PanelClearSettings {
    pub clearDepthStencil: bool,
    pub clearColor: bool,
    pub color: crate::UnityEngine::Color,
}
#[cfg(feature = "UnityEngine+UIElements+PanelClearSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PanelClearSettings =>
    "UnityEngine.UIElements"."PanelClearSettings"
);
#[cfg(feature = "UnityEngine+UIElements+PanelClearSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::PanelClearSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelClearSettings")]
impl crate::UnityEngine::UIElements::PanelClearSettings {}
