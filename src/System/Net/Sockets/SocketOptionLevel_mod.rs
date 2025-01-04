#[cfg(feature = "System+Net+Sockets+SocketOptionLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SocketOptionLevel {
    #[default]
    IP = 0i32,
    IPv6 = 41i32,
    Socket = 65535i32,
    Tcp = 6i32,
    Udp = 17i32,
}
#[cfg(feature = "System+Net+Sockets+SocketOptionLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SocketOptionLevel =>
    "System.Net.Sockets"."SocketOptionLevel"
);
