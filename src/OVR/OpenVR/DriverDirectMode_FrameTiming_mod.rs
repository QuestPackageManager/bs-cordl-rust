#[cfg(feature = "OVR+OpenVR+DriverDirectMode_FrameTiming")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DriverDirectMode_FrameTiming {
    pub m_nSize: u32,
    pub m_nNumFramePresents: u32,
    pub m_nNumMisPresented: u32,
    pub m_nNumDroppedFrames: u32,
    pub m_nReprojectionFlags: u32,
}
#[cfg(feature = "OVR+OpenVR+DriverDirectMode_FrameTiming")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::DriverDirectMode_FrameTiming =>
    "OVR.OpenVR"."DriverDirectMode_FrameTiming"
);
#[cfg(feature = "OVR+OpenVR+DriverDirectMode_FrameTiming")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::DriverDirectMode_FrameTiming {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+DriverDirectMode_FrameTiming")]
impl crate::OVR::OpenVR::DriverDirectMode_FrameTiming {}
