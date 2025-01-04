#[cfg(feature = "System+Net+Sockets+AddressFamily")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AddressFamily {
    #[default]
    AppleTalk = 16i32,
    Atm = 22i32,
    Banyan = 21i32,
    Ccitt = 10i32,
    Chaos = 5i32,
    Cluster = 24i32,
    DataKit = 9i32,
    DataLink = 13i32,
    DecNet = 12i32,
    Ecma = 8i32,
    FireFox = 19i32,
    HyperChannel = 15i32,
    Ieee12844 = 25i32,
    ImpLink = 3i32,
    InterNetwork = 2i32,
    InterNetworkV6 = 23i32,
    Ipx = 6i32,
    Irda = 26i32,
    Iso = 7i32,
    Lat = 14i32,
    Max = 29i32,
    NetBios = 17i32,
    NetworkDesigners = 28i32,
    Pup = 4i32,
    Sna = 11i32,
    Unix = 1i32,
    Unknown = -1i32,
    Unspecified = 0i32,
    VoiceView = 18i32,
}
#[cfg(feature = "System+Net+Sockets+AddressFamily")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::AddressFamily =>
    "System.Net.Sockets"."AddressFamily"
);
