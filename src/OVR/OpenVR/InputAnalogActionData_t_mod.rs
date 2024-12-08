#[cfg(feature = "OVR+OpenVR+InputAnalogActionData_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputAnalogActionData_t {
    pub bActive: bool,
    pub activeOrigin: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub deltaX: f32,
    pub deltaY: f32,
    pub deltaZ: f32,
    pub fUpdateTime: f32,
}
#[cfg(feature = "OVR+OpenVR+InputAnalogActionData_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::InputAnalogActionData_t =>
    "OVR.OpenVR"."InputAnalogActionData_t"
);
#[cfg(feature = "OVR+OpenVR+InputAnalogActionData_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::InputAnalogActionData_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+InputAnalogActionData_t")]
impl crate::OVR::OpenVR::InputAnalogActionData_t {}
