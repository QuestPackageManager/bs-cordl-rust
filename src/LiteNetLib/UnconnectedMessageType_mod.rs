#[cfg(feature = "LiteNetLib+UnconnectedMessageType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnconnectedMessageType {
    BasicMessage = 0i32,
    Broadcast = 1i32,
}
#[cfg(feature = "LiteNetLib+UnconnectedMessageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::UnconnectedMessageType =>
    "LiteNetLib"."UnconnectedMessageType"
);
