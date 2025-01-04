#[cfg(feature = "LiteNetLib+PacketProperty")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PacketProperty {
    #[default]
    Ack = 2u8,
    Broadcast = 11u8,
    Channeled = 1u8,
    ConnectAccept = 6u8,
    ConnectRequest = 5u8,
    Disconnect = 7u8,
    Empty = 17u8,
    InvalidProtocol = 15u8,
    Merged = 12u8,
    MtuCheck = 9u8,
    MtuOk = 10u8,
    NatMessage = 16u8,
    PeerNotFound = 14u8,
    Ping = 3u8,
    Pong = 4u8,
    ShutdownOk = 13u8,
    UnconnectedMessage = 8u8,
    Unreliable = 0u8,
}
#[cfg(feature = "LiteNetLib+PacketProperty")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::PacketProperty => "LiteNetLib"
    ."PacketProperty"
);
