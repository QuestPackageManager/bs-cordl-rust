#[cfg(feature = "OVR+OpenVR+VRMessageOverlayResponse")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VRMessageOverlayResponse {
    #[default]
    ApplicationQuit = 6i32,
    ButtonPress_0 = 0i32,
    ButtonPress_1 = 1i32,
    ButtonPress_2 = 2i32,
    ButtonPress_3 = 3i32,
    CouldntFindOrCreateClientOverlay = 5i32,
    CouldntFindSystemOverlay = 4i32,
}
#[cfg(feature = "OVR+OpenVR+VRMessageOverlayResponse")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VRMessageOverlayResponse =>
    "OVR.OpenVR"."VRMessageOverlayResponse"
);
