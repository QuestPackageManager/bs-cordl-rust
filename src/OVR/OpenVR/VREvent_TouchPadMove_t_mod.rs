#[cfg(feature = "OVR+OpenVR+VREvent_TouchPadMove_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VREvent_TouchPadMove_t {
    pub bFingerDown: bool,
    pub flSecondsFingerDown: f32,
    pub fValueXFirst: f32,
    pub fValueYFirst: f32,
    pub fValueXRaw: f32,
    pub fValueYRaw: f32,
}
#[cfg(feature = "OVR+OpenVR+VREvent_TouchPadMove_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VREvent_TouchPadMove_t =>
    "OVR.OpenVR"."VREvent_TouchPadMove_t"
);
#[cfg(feature = "OVR+OpenVR+VREvent_TouchPadMove_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VREvent_TouchPadMove_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VREvent_TouchPadMove_t")]
impl crate::OVR::OpenVR::VREvent_TouchPadMove_t {}