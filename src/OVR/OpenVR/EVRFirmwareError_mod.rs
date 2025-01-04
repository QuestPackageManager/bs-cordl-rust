#[cfg(feature = "OVR+OpenVR+EVRFirmwareError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRFirmwareError {
    #[default]
    Fail = 2i32,
    None = 0i32,
    Success = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EVRFirmwareError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRFirmwareError => "OVR.OpenVR"
    ."EVRFirmwareError"
);
