#[cfg(feature = "OVR+OpenVR+HmdQuad_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HmdQuad_t {
    pub vCorners0: crate::OVR::OpenVR::HmdVector3_t,
    pub vCorners1: crate::OVR::OpenVR::HmdVector3_t,
    pub vCorners2: crate::OVR::OpenVR::HmdVector3_t,
    pub vCorners3: crate::OVR::OpenVR::HmdVector3_t,
}
#[cfg(feature = "OVR+OpenVR+HmdQuad_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::HmdQuad_t => "OVR.OpenVR"
    ."HmdQuad_t"
);
#[cfg(feature = "OVR+OpenVR+HmdQuad_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::HmdQuad_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+HmdQuad_t")]
impl crate::OVR::OpenVR::HmdQuad_t {}
