#[cfg(feature = "System+Net+NetworkInformation+AixAddressFamily")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AixAddressFamily {
    AF_INET = 2i32,
    AF_INET6 = 24i32,
    AF_LINK = 18i32,
}
#[cfg(feature = "System+Net+NetworkInformation+AixAddressFamily")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixAddressFamily =>
    "System.Net.NetworkInformation"."AixAddressFamily"
);
