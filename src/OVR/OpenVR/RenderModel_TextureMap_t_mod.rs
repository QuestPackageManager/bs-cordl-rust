#[cfg(feature = "OVR+OpenVR+RenderModel_TextureMap_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RenderModel_TextureMap_t {
    pub unWidth: u16,
    pub unHeight: u16,
    pub rubTextureMapData: crate::System::IntPtr,
}
#[cfg(feature = "OVR+OpenVR+RenderModel_TextureMap_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::RenderModel_TextureMap_t =>
    "OVR.OpenVR"."RenderModel_TextureMap_t"
);
#[cfg(feature = "OVR+OpenVR+RenderModel_TextureMap_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::RenderModel_TextureMap_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+RenderModel_TextureMap_t")]
impl crate::OVR::OpenVR::RenderModel_TextureMap_t {}
