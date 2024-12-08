#[cfg(feature = "OVRMaterialData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRMaterialData {
    pub shader: *mut crate::UnityEngine::Shader,
    pub textureId: i32,
    pub texture: crate::GlobalNamespace::OVRTextureData,
    pub baseColorFactor: crate::UnityEngine::Color,
}
#[cfg(feature = "OVRMaterialData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRMaterialData => ""
    ."OVRMaterialData"
);
#[cfg(feature = "OVRMaterialData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRMaterialData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRMaterialData")]
impl crate::GlobalNamespace::OVRMaterialData {}
