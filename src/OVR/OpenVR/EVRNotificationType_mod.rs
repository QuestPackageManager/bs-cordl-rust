#[cfg(feature = "OVR+OpenVR+EVRNotificationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRNotificationType {
    #[default]
    Persistent = 1i32,
    Transient = 0i32,
    Transient_SystemWithUserValue = 2i32,
}
#[cfg(feature = "OVR+OpenVR+EVRNotificationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRNotificationType => "OVR.OpenVR"
    ."EVRNotificationType"
);
