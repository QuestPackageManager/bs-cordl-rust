#[cfg(feature = "OVR+OpenVR+EVRMuraCorrectionMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRMuraCorrectionMode {
    Default = 0i32,
    NoCorrection = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EVRMuraCorrectionMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRMuraCorrectionMode =>
    "OVR.OpenVR"."EVRMuraCorrectionMode"
);
