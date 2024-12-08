#[cfg(feature = "LiteNetLib+DisconnectResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisconnectResult {
    Disconnect = 2i32,
    None = 0i32,
    Reject = 1i32,
}
#[cfg(feature = "LiteNetLib+DisconnectResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::DisconnectResult => "LiteNetLib"
    ."DisconnectResult"
);
