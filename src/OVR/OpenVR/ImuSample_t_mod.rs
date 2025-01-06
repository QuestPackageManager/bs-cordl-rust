#[cfg(feature = "OVR+OpenVR+ImuSample_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ImuSample_t {
    pub fSampleTime: f64,
    pub vAccel: crate::OVR::OpenVR::HmdVector3d_t,
    pub vGyro: crate::OVR::OpenVR::HmdVector3d_t,
    pub unOffScaleFlags: u32,
}
#[cfg(feature = "OVR+OpenVR+ImuSample_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::ImuSample_t => "OVR.OpenVR"
    ."ImuSample_t"
);
#[cfg(feature = "OVR+OpenVR+ImuSample_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::ImuSample_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+ImuSample_t")]
impl crate::OVR::OpenVR::ImuSample_t {}
