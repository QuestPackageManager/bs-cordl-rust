#[cfg(feature = "System+Net+Sockets+SocketOperation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SocketOperation {
    #[default]
    Accept = 0i32,
    AcceptReceive = 9i32,
    Connect = 1i32,
    Disconnect = 8i32,
    Receive = 2i32,
    ReceiveFrom = 3i32,
    ReceiveGeneric = 10i32,
    RecvJustCallback = 6i32,
    Send = 4i32,
    SendGeneric = 11i32,
    SendJustCallback = 7i32,
    SendTo = 5i32,
}
#[cfg(feature = "System+Net+Sockets+SocketOperation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SocketOperation =>
    "System.Net.Sockets"."SocketOperation"
);
