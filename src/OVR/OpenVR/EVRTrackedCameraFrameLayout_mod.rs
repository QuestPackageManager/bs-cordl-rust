#[cfg(feature = "OVR+OpenVR+EVRTrackedCameraFrameLayout")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRTrackedCameraFrameLayout {
    #[default]
    HorizontalLayout = 32i32,
    Mono = 1i32,
    Stereo = 2i32,
    VerticalLayout = 16i32,
}
#[cfg(feature = "OVR+OpenVR+EVRTrackedCameraFrameLayout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRTrackedCameraFrameLayout =>
    "OVR.OpenVR"."EVRTrackedCameraFrameLayout"
);
