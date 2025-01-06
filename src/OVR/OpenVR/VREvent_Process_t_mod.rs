#[cfg(feature = "OVR+OpenVR+VREvent_Process_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VREvent_Process_t {
    pub pid: u32,
    pub oldPid: u32,
    pub bForced: bool,
}
#[cfg(feature = "OVR+OpenVR+VREvent_Process_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VREvent_Process_t => "OVR.OpenVR"
    ."VREvent_Process_t"
);
#[cfg(feature = "OVR+OpenVR+VREvent_Process_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VREvent_Process_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VREvent_Process_t")]
impl crate::OVR::OpenVR::VREvent_Process_t {}
