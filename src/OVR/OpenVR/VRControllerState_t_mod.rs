#[cfg(feature = "OVR+OpenVR+VRControllerState_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VRControllerState_t {
    pub unPacketNum: u32,
    pub ulButtonPressed: u64,
    pub ulButtonTouched: u64,
    pub rAxis0: crate::OVR::OpenVR::VRControllerAxis_t,
    pub rAxis1: crate::OVR::OpenVR::VRControllerAxis_t,
    pub rAxis2: crate::OVR::OpenVR::VRControllerAxis_t,
    pub rAxis3: crate::OVR::OpenVR::VRControllerAxis_t,
    pub rAxis4: crate::OVR::OpenVR::VRControllerAxis_t,
}
#[cfg(feature = "OVR+OpenVR+VRControllerState_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VRControllerState_t => "OVR.OpenVR"
    ."VRControllerState_t"
);
#[cfg(feature = "OVR+OpenVR+VRControllerState_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VRControllerState_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VRControllerState_t")]
impl crate::OVR::OpenVR::VRControllerState_t {}
