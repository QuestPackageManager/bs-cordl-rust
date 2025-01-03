#[cfg(feature = "OVR+OpenVR+VRActiveActionSet_t")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct VRActiveActionSet_t {
    pub ulActionSet: u64,
    pub ulRestrictedToDevice: u64,
    pub ulSecondaryActionSet: u64,
    pub unPadding: u32,
    pub nPriority: i32,
}
#[cfg(feature = "OVR+OpenVR+VRActiveActionSet_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VRActiveActionSet_t => "OVR.OpenVR"
    ."VRActiveActionSet_t"
);
#[cfg(feature = "OVR+OpenVR+VRActiveActionSet_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VRActiveActionSet_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VRActiveActionSet_t")]
impl crate::OVR::OpenVR::VRActiveActionSet_t {}
