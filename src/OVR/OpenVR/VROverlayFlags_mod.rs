#[cfg(feature = "OVR+OpenVR+VROverlayFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VROverlayFlags {
    AcceptsGamepadEvents = 4i32,
    Curved = 1i32,
    NoDashboardTab = 3i32,
    None = 0i32,
    Panorama = 12i32,
    RGSS4X = 2i32,
    SendVRScrollEvents = 6i32,
    SendVRTouchpadEvents = 7i32,
    ShowGamepadFocus = 5i32,
    ShowTouchPadScrollWheel = 8i32,
    SideBySide_Crossed = 11i32,
    SideBySide_Parallel = 10i32,
    SortWithNonSceneOverlays = 14i32,
    StereoPanorama = 13i32,
    TransferOwnershipToInternalProcess = 9i32,
    VisibleInDashboard = 15i32,
}
#[cfg(feature = "OVR+OpenVR+VROverlayFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VROverlayFlags => "OVR.OpenVR"
    ."VROverlayFlags"
);
