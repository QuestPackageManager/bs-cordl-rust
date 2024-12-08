#[cfg(feature = "OVR+OpenVR+HmdQuaternion_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HmdQuaternion_t {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[cfg(feature = "OVR+OpenVR+HmdQuaternion_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::HmdQuaternion_t => "OVR.OpenVR"
    ."HmdQuaternion_t"
);
#[cfg(feature = "OVR+OpenVR+HmdQuaternion_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::HmdQuaternion_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+HmdQuaternion_t")]
impl crate::OVR::OpenVR::HmdQuaternion_t {}
