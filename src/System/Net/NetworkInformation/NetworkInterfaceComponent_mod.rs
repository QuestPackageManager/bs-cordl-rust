#[cfg(feature = "System+Net+NetworkInformation+NetworkInterfaceComponent")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkInterfaceComponent {
    IPv4 = 0i32,
    IPv6 = 1i32,
}
#[cfg(feature = "System+Net+NetworkInformation+NetworkInterfaceComponent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::NetworkInterfaceComponent =>
    "System.Net.NetworkInformation"."NetworkInterfaceComponent"
);
