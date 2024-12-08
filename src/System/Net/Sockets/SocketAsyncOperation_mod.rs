#[cfg(feature = "System+Net+Sockets+SocketAsyncOperation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketAsyncOperation {
    Accept = 1i32,
    Connect = 2i32,
    Disconnect = 3i32,
    None = 0i32,
    Receive = 4i32,
    ReceiveFrom = 5i32,
    ReceiveMessageFrom = 6i32,
    Send = 7i32,
    SendPackets = 8i32,
    SendTo = 9i32,
}
#[cfg(feature = "System+Net+Sockets+SocketAsyncOperation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SocketAsyncOperation =>
    "System.Net.Sockets"."SocketAsyncOperation"
);
