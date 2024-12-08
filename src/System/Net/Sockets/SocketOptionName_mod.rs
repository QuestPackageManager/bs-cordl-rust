#[cfg(feature = "System+Net+Sockets+SocketOptionName")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketOptionName {
    AcceptConnection = 2i32,
    AddMembership = 12i32,
    AddSourceMembership = 15i32,
    BlockSource = 17i32,
    Broadcast = 32i32,
    ChecksumCoverage = 20i32,
    Debug = 1i32,
    DontFragment = 14i32,
    DontLinger = -129i32,
    DontRoute = 16i32,
    DropMembership = 13i32,
    Error = 4103i32,
    ExclusiveAddressUse = -5i32,
    HopLimit = 21i32,
    IPProtectionLevel = 23i32,
    IPv6Only = 27i32,
    IpTimeToLive = 4i32,
    KeepAlive = 8i32,
    Linger = 128i32,
    MaxConnections = 2147483647i32,
    MulticastInterface = 9i32,
    MulticastLoopback = 11i32,
    MulticastTimeToLive = 10i32,
    OutOfBandInline = 256i32,
    PacketInformation = 19i32,
    ReceiveBuffer = 4098i32,
    ReceiveLowWater = 4100i32,
    ReceiveTimeout = 4102i32,
    ReuseUnicastPort = 12295i32,
    SendBuffer = 4097i32,
    SendLowWater = 4099i32,
    SendTimeout = 4101i32,
    Type = 4104i32,
    TypeOfService = 3i32,
    UnblockSource = 18i32,
    UpdateAcceptContext = 28683i32,
    UpdateConnectContext = 28688i32,
    UseLoopback = 64i32,
}
#[cfg(feature = "System+Net+Sockets+SocketOptionName")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SocketOptionName =>
    "System.Net.Sockets"."SocketOptionName"
);
