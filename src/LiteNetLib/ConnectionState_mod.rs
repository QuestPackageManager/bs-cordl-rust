#[cfg(feature = "LiteNetLib+ConnectionState")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Any = 14u8,
    Connected = 4u8,
    Disconnected = 16u8,
    Outgoing = 2u8,
    ShutdownRequested = 8u8,
}
#[cfg(feature = "LiteNetLib+ConnectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::ConnectionState => "LiteNetLib"
    ."ConnectionState"
);
