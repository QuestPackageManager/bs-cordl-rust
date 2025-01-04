#[cfg(feature = "System+Net+Sockets+SocketShutdown")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SocketShutdown {
    #[default]
    Both = 2i32,
    Receive = 0i32,
    Send = 1i32,
}
#[cfg(feature = "System+Net+Sockets+SocketShutdown")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SocketShutdown =>
    "System.Net.Sockets"."SocketShutdown"
);
