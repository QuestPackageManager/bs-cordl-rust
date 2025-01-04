#[cfg(feature = "OVR+OpenVR+EOverlayDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EOverlayDirection {
    #[default]
    Count = 4i32,
    Down = 1i32,
    Left = 2i32,
    Right = 3i32,
    Up = 0i32,
}
#[cfg(feature = "OVR+OpenVR+EOverlayDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EOverlayDirection => "OVR.OpenVR"
    ."EOverlayDirection"
);
