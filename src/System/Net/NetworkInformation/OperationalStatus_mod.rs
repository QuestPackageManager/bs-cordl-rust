#[cfg(feature = "System+Net+NetworkInformation+OperationalStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalStatus {
    Dormant = 5i32,
    Down = 2i32,
    LowerLayerDown = 7i32,
    NotPresent = 6i32,
    Testing = 3i32,
    Unknown = 4i32,
    Up = 1i32,
}
#[cfg(feature = "System+Net+NetworkInformation+OperationalStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::OperationalStatus =>
    "System.Net.NetworkInformation"."OperationalStatus"
);
