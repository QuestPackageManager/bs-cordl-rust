#[cfg(feature = "OVR+OpenVR+EVRNotificationError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRNotificationError {
    #[default]
    InvalidNotificationId = 100i32,
    InvalidOverlayHandle = 102i32,
    NotificationQueueFull = 101i32,
    OK = 0i32,
    SystemWithUserValueAlreadyExists = 103i32,
}
#[cfg(feature = "OVR+OpenVR+EVRNotificationError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRNotificationError =>
    "OVR.OpenVR"."EVRNotificationError"
);
