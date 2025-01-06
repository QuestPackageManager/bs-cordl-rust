#[cfg(feature = "OVR+OpenVR+VRTextureDepthInfo_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VRTextureDepthInfo_t {
    pub handle: crate::System::IntPtr,
    pub mProjection: crate::OVR::OpenVR::HmdMatrix44_t,
    pub vRange: crate::OVR::OpenVR::HmdVector2_t,
}
#[cfg(feature = "OVR+OpenVR+VRTextureDepthInfo_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VRTextureDepthInfo_t =>
    "OVR.OpenVR"."VRTextureDepthInfo_t"
);
#[cfg(feature = "OVR+OpenVR+VRTextureDepthInfo_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VRTextureDepthInfo_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VRTextureDepthInfo_t")]
impl crate::OVR::OpenVR::VRTextureDepthInfo_t {}
