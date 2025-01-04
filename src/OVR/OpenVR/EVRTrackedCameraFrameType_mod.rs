#[cfg(feature = "OVR+OpenVR+EVRTrackedCameraFrameType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRTrackedCameraFrameType {
    #[default]
    Distorted = 0i32,
    MAX_CAMERA_FRAME_TYPES = 3i32,
    MaximumUndistorted = 2i32,
    Undistorted = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EVRTrackedCameraFrameType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRTrackedCameraFrameType =>
    "OVR.OpenVR"."EVRTrackedCameraFrameType"
);
