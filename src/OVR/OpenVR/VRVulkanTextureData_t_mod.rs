#[cfg(feature = "OVR+OpenVR+VRVulkanTextureData_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VRVulkanTextureData_t {
    pub m_nImage: u64,
    pub m_pDevice: crate::System::IntPtr,
    pub m_pPhysicalDevice: crate::System::IntPtr,
    pub m_pInstance: crate::System::IntPtr,
    pub m_pQueue: crate::System::IntPtr,
    pub m_nQueueFamilyIndex: u32,
    pub m_nWidth: u32,
    pub m_nHeight: u32,
    pub m_nFormat: u32,
    pub m_nSampleCount: u32,
}
#[cfg(feature = "OVR+OpenVR+VRVulkanTextureData_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VRVulkanTextureData_t =>
    "OVR.OpenVR"."VRVulkanTextureData_t"
);
#[cfg(feature = "OVR+OpenVR+VRVulkanTextureData_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VRVulkanTextureData_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VRVulkanTextureData_t")]
impl crate::OVR::OpenVR::VRVulkanTextureData_t {}
