#[cfg(feature = "System+Net+Sockets+SocketFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketFlags {
    Broadcast = 1024i32,
    ControlDataTruncated = 512i32,
    DontRoute = 4i32,
    MaxIOVectorLength = 16i32,
    Multicast = 2048i32,
    None = 0i32,
    OutOfBand = 1i32,
    Partial = 32768i32,
    Peek = 2i32,
    Truncated = 256i32,
}
#[cfg(feature = "System+Net+Sockets+SocketFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SocketFlags =>
    "System.Net.Sockets"."SocketFlags"
);
