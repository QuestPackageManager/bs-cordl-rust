#[cfg(feature = "System+Net+Sockets+ProtocolType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolType {
    Ggp = 3i32,
    IP = 0i32,
    IPSecAuthenticationHeader = 51i32,
    IPSecEncapsulatingSecurityPayload = 50i32,
    IPv4 = 4i32,
    IPv6 = 41i32,
    IPv6DestinationOptions = 60i32,
    IPv6FragmentHeader = 44i32,
    IPv6NoNextHeader = 59i32,
    IPv6RoutingHeader = 43i32,
    Icmp = 1i32,
    IcmpV6 = 58i32,
    Idp = 22i32,
    Igmp = 2i32,
    Ipx = 1000i32,
    ND = 77i32,
    Pup = 12i32,
    Raw = 255i32,
    Spx = 1256i32,
    SpxII = 1257i32,
    Tcp = 6i32,
    Udp = 17i32,
    Unknown = -1i32,
}
#[cfg(feature = "System+Net+Sockets+ProtocolType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::ProtocolType =>
    "System.Net.Sockets"."ProtocolType"
);
