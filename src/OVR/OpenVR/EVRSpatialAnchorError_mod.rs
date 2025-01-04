#[cfg(feature = "OVR+OpenVR+EVRSpatialAnchorError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRSpatialAnchorError {
    #[default]
    ArrayTooSmall = 3i32,
    DescriptorTooLong = 9i32,
    Internal = 1i32,
    InvalidArgument = 12i32,
    InvalidDescriptorChar = 4i32,
    NoRoomCalibration = 11i32,
    NotAvailableInThisUniverse = 6i32,
    NotYetAvailable = 5i32,
    PermanentlyUnavailable = 7i32,
    Success = 0i32,
    Unknown = 10i32,
    UnknownDriver = 13i32,
    UnknownHandle = 2i32,
    WrongDriver = 8i32,
}
#[cfg(feature = "OVR+OpenVR+EVRSpatialAnchorError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRSpatialAnchorError =>
    "OVR.OpenVR"."EVRSpatialAnchorError"
);
