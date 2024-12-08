#[cfg(feature = "OVR+OpenVR+CameraVideoStreamFrameHeader_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CameraVideoStreamFrameHeader_t {
    pub eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
    pub nWidth: u32,
    pub nHeight: u32,
    pub nBytesPerPixel: u32,
    pub nFrameSequence: u32,
    pub standingTrackedDevicePose: crate::OVR::OpenVR::TrackedDevicePose_t,
}
#[cfg(feature = "OVR+OpenVR+CameraVideoStreamFrameHeader_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CameraVideoStreamFrameHeader_t =>
    "OVR.OpenVR"."CameraVideoStreamFrameHeader_t"
);
#[cfg(feature = "OVR+OpenVR+CameraVideoStreamFrameHeader_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+CameraVideoStreamFrameHeader_t")]
impl crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t {}
