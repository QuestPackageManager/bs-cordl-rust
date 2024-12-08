#[cfg(feature = "System+Net+Sockets+SocketType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Dgram = 2i32,
    Raw = 3i32,
    Rdm = 4i32,
    Seqpacket = 5i32,
    Stream = 1i32,
    Unknown = -1i32,
}
#[cfg(feature = "System+Net+Sockets+SocketType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SocketType =>
    "System.Net.Sockets"."SocketType"
);
