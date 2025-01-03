#[cfg(feature = "UnityEngine+Rendering+BatchFilterSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BatchFilterSettings {
    pub renderingLayerMask: u32,
    pub layer: u8,
    pub m_motionMode: u8,
    pub m_shadowMode: u8,
    pub m_receiveShadows: u8,
    pub m_staticShadowCaster: u8,
    pub m_allDepthSorted: u8,
}
#[cfg(feature = "UnityEngine+Rendering+BatchFilterSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchFilterSettings =>
    "UnityEngine.Rendering"."BatchFilterSettings"
);
#[cfg(feature = "UnityEngine+Rendering+BatchFilterSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchFilterSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchFilterSettings")]
impl crate::UnityEngine::Rendering::BatchFilterSettings {}
