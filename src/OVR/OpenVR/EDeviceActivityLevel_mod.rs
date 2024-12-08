#[cfg(feature = "OVR+OpenVR+EDeviceActivityLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EDeviceActivityLevel {
    k_EDeviceActivityLevel_Idle = 0i32,
    k_EDeviceActivityLevel_Standby = 3i32,
    k_EDeviceActivityLevel_Unknown = -1i32,
    k_EDeviceActivityLevel_UserInteraction = 1i32,
    k_EDeviceActivityLevel_UserInteraction_Timeout = 2i32,
}
#[cfg(feature = "OVR+OpenVR+EDeviceActivityLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EDeviceActivityLevel =>
    "OVR.OpenVR"."EDeviceActivityLevel"
);
