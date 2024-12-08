#[cfg(feature = "OVR+OpenVR+EVRControllerAxisType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRControllerAxisType {
    k_eControllerAxis_Joystick = 2i32,
    k_eControllerAxis_None = 0i32,
    k_eControllerAxis_TrackPad = 1i32,
    k_eControllerAxis_Trigger = 3i32,
}
#[cfg(feature = "OVR+OpenVR+EVRControllerAxisType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRControllerAxisType =>
    "OVR.OpenVR"."EVRControllerAxisType"
);
