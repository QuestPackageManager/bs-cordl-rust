#[cfg(feature = "OVR+OpenVR+VREvent_ScreenshotProgress_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VREvent_ScreenshotProgress_t {
    pub progress: f32,
}
#[cfg(feature = "OVR+OpenVR+VREvent_ScreenshotProgress_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VREvent_ScreenshotProgress_t =>
    "OVR.OpenVR"."VREvent_ScreenshotProgress_t"
);
#[cfg(feature = "OVR+OpenVR+VREvent_ScreenshotProgress_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VREvent_ScreenshotProgress_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VREvent_ScreenshotProgress_t")]
impl crate::OVR::OpenVR::VREvent_ScreenshotProgress_t {}
