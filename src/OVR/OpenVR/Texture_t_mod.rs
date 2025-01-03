#[cfg(feature = "OVR+OpenVR+Texture_t")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Texture_t {
    pub handle: crate::System::IntPtr,
    pub eType: crate::OVR::OpenVR::ETextureType,
    pub eColorSpace: crate::OVR::OpenVR::EColorSpace,
}
#[cfg(feature = "OVR+OpenVR+Texture_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::Texture_t => "OVR.OpenVR"
    ."Texture_t"
);
#[cfg(feature = "OVR+OpenVR+Texture_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::Texture_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+Texture_t")]
impl crate::OVR::OpenVR::Texture_t {}
