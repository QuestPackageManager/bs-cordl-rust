#[cfg(feature = "OVR+OpenVR+HmdMatrix34_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HmdMatrix34_t {
    pub m0: f32,
    pub m1: f32,
    pub m2: f32,
    pub m3: f32,
    pub m4: f32,
    pub m5: f32,
    pub m6: f32,
    pub m7: f32,
    pub m8: f32,
    pub m9: f32,
    pub m10: f32,
    pub m11: f32,
}
#[cfg(feature = "OVR+OpenVR+HmdMatrix34_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::HmdMatrix34_t => "OVR.OpenVR"
    ."HmdMatrix34_t"
);
#[cfg(feature = "OVR+OpenVR+HmdMatrix34_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::HmdMatrix34_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+HmdMatrix34_t")]
impl crate::OVR::OpenVR::HmdMatrix34_t {}
