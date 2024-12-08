#[cfg(feature = "OVR+OpenVR+COpenVRContext")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct COpenVRContext {
    pub m_pVRSystem: crate::System::IntPtr,
    pub m_pVRChaperone: crate::System::IntPtr,
    pub m_pVRChaperoneSetup: crate::System::IntPtr,
    pub m_pVRCompositor: crate::System::IntPtr,
    pub m_pVROverlay: crate::System::IntPtr,
    pub m_pVRResources: crate::System::IntPtr,
    pub m_pVRRenderModels: crate::System::IntPtr,
    pub m_pVRExtendedDisplay: crate::System::IntPtr,
    pub m_pVRSettings: crate::System::IntPtr,
    pub m_pVRApplications: crate::System::IntPtr,
    pub m_pVRTrackedCamera: crate::System::IntPtr,
    pub m_pVRScreenshots: crate::System::IntPtr,
    pub m_pVRDriverManager: crate::System::IntPtr,
    pub m_pVRInput: crate::System::IntPtr,
    pub m_pVRIOBuffer: crate::System::IntPtr,
    pub m_pVRSpatialAnchors: crate::System::IntPtr,
}
#[cfg(feature = "OVR+OpenVR+COpenVRContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::COpenVRContext => "OVR.OpenVR"
    ."COpenVRContext"
);
#[cfg(feature = "OVR+OpenVR+COpenVRContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::COpenVRContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+COpenVRContext")]
impl crate::OVR::OpenVR::COpenVRContext {}
