#[cfg(feature = "UnityEngine+LightBakingOutput")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LightBakingOutput {
    pub probeOcclusionLightIndex: i32,
    pub occlusionMaskChannel: i32,
    pub lightmapBakeType: crate::UnityEngine::LightmapBakeType,
    pub mixedLightingMode: crate::UnityEngine::MixedLightingMode,
    pub isBaked: bool,
}
#[cfg(feature = "UnityEngine+LightBakingOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LightBakingOutput => "UnityEngine"
    ."LightBakingOutput"
);
#[cfg(feature = "UnityEngine+LightBakingOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::LightBakingOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+LightBakingOutput")]
impl crate::UnityEngine::LightBakingOutput {}
