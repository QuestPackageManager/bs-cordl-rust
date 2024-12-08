#[cfg(feature = "LiteNetLib+ConnectRequestResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectRequestResult {
    NewConnection = 3i32,
    None = 0i32,
    P2PLose = 1i32,
    Reconnection = 2i32,
}
#[cfg(feature = "LiteNetLib+ConnectRequestResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::ConnectRequestResult => "LiteNetLib"
    ."ConnectRequestResult"
);
