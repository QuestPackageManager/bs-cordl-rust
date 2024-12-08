#[cfg(feature = "OVR+OpenVR+VREvent_InputActionManifestLoad_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VREvent_InputActionManifestLoad_t {
    pub pathAppKey: u64,
    pub pathMessage: u64,
    pub pathMessageParam: u64,
    pub pathManifestPath: u64,
}
#[cfg(feature = "OVR+OpenVR+VREvent_InputActionManifestLoad_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VREvent_InputActionManifestLoad_t
    => "OVR.OpenVR"."VREvent_InputActionManifestLoad_t"
);
#[cfg(feature = "OVR+OpenVR+VREvent_InputActionManifestLoad_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VREvent_InputActionManifestLoad_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VREvent_InputActionManifestLoad_t")]
impl crate::OVR::OpenVR::VREvent_InputActionManifestLoad_t {}
