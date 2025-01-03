#[cfg(feature = "OVR+OpenVR+HmdColor_t")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HmdColor_t {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[cfg(feature = "OVR+OpenVR+HmdColor_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::HmdColor_t => "OVR.OpenVR"
    ."HmdColor_t"
);
#[cfg(feature = "OVR+OpenVR+HmdColor_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::HmdColor_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+HmdColor_t")]
impl crate::OVR::OpenVR::HmdColor_t {}
