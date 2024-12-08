#[cfg(feature = "OVR+OpenVR+Compositor_FrameTiming")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Compositor_FrameTiming {
    pub m_nSize: u32,
    pub m_nFrameIndex: u32,
    pub m_nNumFramePresents: u32,
    pub m_nNumMisPresented: u32,
    pub m_nNumDroppedFrames: u32,
    pub m_nReprojectionFlags: u32,
    pub m_flSystemTimeInSeconds: f64,
    pub m_flPreSubmitGpuMs: f32,
    pub m_flPostSubmitGpuMs: f32,
    pub m_flTotalRenderGpuMs: f32,
    pub m_flCompositorRenderGpuMs: f32,
    pub m_flCompositorRenderCpuMs: f32,
    pub m_flCompositorIdleCpuMs: f32,
    pub m_flClientFrameIntervalMs: f32,
    pub m_flPresentCallCpuMs: f32,
    pub m_flWaitForPresentCpuMs: f32,
    pub m_flSubmitFrameMs: f32,
    pub m_flWaitGetPosesCalledMs: f32,
    pub m_flNewPosesReadyMs: f32,
    pub m_flNewFrameReadyMs: f32,
    pub m_flCompositorUpdateStartMs: f32,
    pub m_flCompositorUpdateEndMs: f32,
    pub m_flCompositorRenderStartMs: f32,
    pub m_HmdPose: crate::OVR::OpenVR::TrackedDevicePose_t,
}
#[cfg(feature = "OVR+OpenVR+Compositor_FrameTiming")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::Compositor_FrameTiming =>
    "OVR.OpenVR"."Compositor_FrameTiming"
);
#[cfg(feature = "OVR+OpenVR+Compositor_FrameTiming")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::Compositor_FrameTiming {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+Compositor_FrameTiming")]
impl crate::OVR::OpenVR::Compositor_FrameTiming {}