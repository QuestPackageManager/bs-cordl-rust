#[cfg(feature = "LiteNetLib+DisconnectReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisconnectReason {
    #[default]
    ConnectionFailed = 0i32,
    ConnectionRejected = 6i32,
    DisconnectPeerCalled = 5i32,
    HostUnreachable = 2i32,
    InvalidProtocol = 7i32,
    NetworkUnreachable = 3i32,
    PeerToPeerConnection = 10i32,
    Reconnect = 9i32,
    RemoteConnectionClose = 4i32,
    Timeout = 1i32,
    UnknownHost = 8i32,
}
#[cfg(feature = "LiteNetLib+DisconnectReason")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::DisconnectReason => "LiteNetLib"
    ."DisconnectReason"
);
