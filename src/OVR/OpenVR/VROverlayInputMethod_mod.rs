#[cfg(feature = "OVR+OpenVR+VROverlayInputMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VROverlayInputMethod {
    DualAnalog = 2i32,
    Mouse = 1i32,
    None = 0i32,
}
#[cfg(feature = "OVR+OpenVR+VROverlayInputMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VROverlayInputMethod =>
    "OVR.OpenVR"."VROverlayInputMethod"
);
