#[cfg(feature = "OVR+OpenVR+Compositor_CumulativeStats")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Compositor_CumulativeStats {
    pub m_nPid: u32,
    pub m_nNumFramePresents: u32,
    pub m_nNumDroppedFrames: u32,
    pub m_nNumReprojectedFrames: u32,
    pub m_nNumFramePresentsOnStartup: u32,
    pub m_nNumDroppedFramesOnStartup: u32,
    pub m_nNumReprojectedFramesOnStartup: u32,
    pub m_nNumLoading: u32,
    pub m_nNumFramePresentsLoading: u32,
    pub m_nNumDroppedFramesLoading: u32,
    pub m_nNumReprojectedFramesLoading: u32,
    pub m_nNumTimedOut: u32,
    pub m_nNumFramePresentsTimedOut: u32,
    pub m_nNumDroppedFramesTimedOut: u32,
    pub m_nNumReprojectedFramesTimedOut: u32,
}
#[cfg(feature = "OVR+OpenVR+Compositor_CumulativeStats")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::Compositor_CumulativeStats =>
    "OVR.OpenVR"."Compositor_CumulativeStats"
);
#[cfg(feature = "OVR+OpenVR+Compositor_CumulativeStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::Compositor_CumulativeStats {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+Compositor_CumulativeStats")]
impl crate::OVR::OpenVR::Compositor_CumulativeStats {}
