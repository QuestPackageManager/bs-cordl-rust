#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionParams_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VROverlayIntersectionParams_t {
    pub vSource: crate::OVR::OpenVR::HmdVector3_t,
    pub vDirection: crate::OVR::OpenVR::HmdVector3_t,
    pub eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
}
#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionParams_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VROverlayIntersectionParams_t =>
    "OVR.OpenVR"."VROverlayIntersectionParams_t"
);
#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionParams_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VROverlayIntersectionParams_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionParams_t")]
impl crate::OVR::OpenVR::VROverlayIntersectionParams_t {}
