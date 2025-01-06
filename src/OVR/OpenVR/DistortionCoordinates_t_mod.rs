#[cfg(feature = "OVR+OpenVR+DistortionCoordinates_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DistortionCoordinates_t {
    pub rfRed0: f32,
    pub rfRed1: f32,
    pub rfGreen0: f32,
    pub rfGreen1: f32,
    pub rfBlue0: f32,
    pub rfBlue1: f32,
}
#[cfg(feature = "OVR+OpenVR+DistortionCoordinates_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::DistortionCoordinates_t =>
    "OVR.OpenVR"."DistortionCoordinates_t"
);
#[cfg(feature = "OVR+OpenVR+DistortionCoordinates_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::DistortionCoordinates_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+DistortionCoordinates_t")]
impl crate::OVR::OpenVR::DistortionCoordinates_t {}
