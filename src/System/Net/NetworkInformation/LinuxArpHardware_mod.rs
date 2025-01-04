#[cfg(feature = "System+Net+NetworkInformation+LinuxArpHardware")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinuxArpHardware {
    #[default]
    ATM = 19i32,
    CSLIP = 257i32,
    CSLIP6 = 259i32,
    EETHER = 2i32,
    ETHER = 1i32,
    FDDI = 774i32,
    IP6GRE = 823i32,
    IPDDP = 777i32,
    IPGRE = 778i32,
    LOOPBACK = 772i32,
    PPP = 512i32,
    PRONET = 4i32,
    SIT = 776i32,
    SLIP = 256i32,
    SLIP6 = 258i32,
    TUNNEL = 768i32,
    TUNNEL6 = 769i32,
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxArpHardware")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::LinuxArpHardware =>
    "System.Net.NetworkInformation"."LinuxArpHardware"
);
