#[cfg(feature = "OVR+OpenVR+InputDigitalActionData_t")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputDigitalActionData_t {
    pub bActive: bool,
    pub activeOrigin: u64,
    pub bState: bool,
    pub bChanged: bool,
    pub fUpdateTime: f32,
}
#[cfg(feature = "OVR+OpenVR+InputDigitalActionData_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::InputDigitalActionData_t =>
    "OVR.OpenVR"."InputDigitalActionData_t"
);
#[cfg(feature = "OVR+OpenVR+InputDigitalActionData_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::InputDigitalActionData_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+InputDigitalActionData_t")]
impl crate::OVR::OpenVR::InputDigitalActionData_t {}
