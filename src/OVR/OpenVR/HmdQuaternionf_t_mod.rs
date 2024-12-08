#[cfg(feature = "OVR+OpenVR+HmdQuaternionf_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HmdQuaternionf_t {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[cfg(feature = "OVR+OpenVR+HmdQuaternionf_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::HmdQuaternionf_t => "OVR.OpenVR"
    ."HmdQuaternionf_t"
);
#[cfg(feature = "OVR+OpenVR+HmdQuaternionf_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::HmdQuaternionf_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+HmdQuaternionf_t")]
impl crate::OVR::OpenVR::HmdQuaternionf_t {}
