#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionResults_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VROverlayIntersectionResults_t {
    pub vPoint: crate::OVR::OpenVR::HmdVector3_t,
    pub vNormal: crate::OVR::OpenVR::HmdVector3_t,
    pub vUVs: crate::OVR::OpenVR::HmdVector2_t,
    pub fDistance: f32,
}
#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionResults_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VROverlayIntersectionResults_t =>
    "OVR.OpenVR"."VROverlayIntersectionResults_t"
);
#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionResults_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VROverlayIntersectionResults_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionResults_t")]
impl crate::OVR::OpenVR::VROverlayIntersectionResults_t {}
