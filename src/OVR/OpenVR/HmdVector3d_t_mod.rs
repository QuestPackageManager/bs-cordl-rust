#[cfg(feature = "OVR+OpenVR+HmdVector3d_t")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HmdVector3d_t {
    pub v0: f64,
    pub v1: f64,
    pub v2: f64,
}
#[cfg(feature = "OVR+OpenVR+HmdVector3d_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::HmdVector3d_t => "OVR.OpenVR"
    ."HmdVector3d_t"
);
#[cfg(feature = "OVR+OpenVR+HmdVector3d_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::HmdVector3d_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+HmdVector3d_t")]
impl crate::OVR::OpenVR::HmdVector3d_t {}
