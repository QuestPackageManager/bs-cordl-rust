#[cfg(feature = "System+Net+NetworkInformation+NetworkInterfaceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkInterfaceType {
    AsymmetricDsl = 94i32,
    Atm = 37i32,
    BasicIsdn = 20i32,
    Ethernet = 6i32,
    Ethernet3Megabit = 26i32,
    FastEthernetFx = 69i32,
    FastEthernetT = 62i32,
    Fddi = 15i32,
    GenericModem = 48i32,
    GigabitEthernet = 117i32,
    HighPerformanceSerialBus = 144i32,
    IPOverAtm = 114i32,
    Isdn = 63i32,
    Loopback = 24i32,
    MultiRateSymmetricDsl = 143i32,
    Ppp = 23i32,
    PrimaryIsdn = 21i32,
    RateAdaptDsl = 95i32,
    Slip = 28i32,
    SymmetricDsl = 96i32,
    TokenRing = 9i32,
    Tunnel = 131i32,
    Unknown = 1i32,
    VeryHighSpeedDsl = 97i32,
    Wireless80211 = 71i32,
    Wman = 237i32,
    Wwanpp = 243i32,
    Wwanpp2 = 244i32,
}
#[cfg(feature = "System+Net+NetworkInformation+NetworkInterfaceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::NetworkInterfaceType =>
    "System.Net.NetworkInformation"."NetworkInterfaceType"
);
