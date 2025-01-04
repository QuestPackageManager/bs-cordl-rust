#[cfg(feature = "System+Net+NetworkInformation+MacOsArpHardware")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacOsArpHardware {
    #[default]
    ATM = 37i32,
    ETHER = 6i32,
    FDDI = 15i32,
    LOOPBACK = 24i32,
    PPP = 23i32,
    SLIP = 28i32,
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsArpHardware")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::MacOsArpHardware =>
    "System.Net.NetworkInformation"."MacOsArpHardware"
);
