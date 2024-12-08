#[cfg(feature = "OVR+OpenVR+VROverlayTransformType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VROverlayTransformType {
    VROverlayTransform_Absolute = 0i32,
    VROverlayTransform_SystemOverlay = 2i32,
    VROverlayTransform_TrackedComponent = 3i32,
    VROverlayTransform_TrackedDeviceRelative = 1i32,
}
#[cfg(feature = "OVR+OpenVR+VROverlayTransformType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VROverlayTransformType =>
    "OVR.OpenVR"."VROverlayTransformType"
);
