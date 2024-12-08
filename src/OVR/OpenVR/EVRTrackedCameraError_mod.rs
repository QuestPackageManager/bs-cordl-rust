#[cfg(feature = "OVR+OpenVR+EVRTrackedCameraError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRTrackedCameraError {
    FailedToGetGLTextureId = 111i32,
    FrameBufferingFailure = 107i32,
    IPCFailure = 104i32,
    InvalidArgument = 114i32,
    InvalidFrameBufferSize = 115i32,
    InvalidFrameHeaderVersion = 102i32,
    InvalidGLTextureId = 109i32,
    InvalidHandle = 101i32,
    InvalidSharedTextureHandle = 110i32,
    NoFrameAvailable = 113i32,
    None = 0i32,
    NotSupportedForThisDevice = 105i32,
    OperationFailed = 100i32,
    OutOfHandles = 103i32,
    SharedMemoryFailure = 106i32,
    SharedTextureFailure = 112i32,
    StreamSetupFailure = 108i32,
}
#[cfg(feature = "OVR+OpenVR+EVRTrackedCameraError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRTrackedCameraError =>
    "OVR.OpenVR"."EVRTrackedCameraError"
);
