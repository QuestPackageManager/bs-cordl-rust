#[cfg(feature = "LiteNetLib+ShutdownResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownResult {
    None = 0i32,
    Success = 1i32,
    WasConnected = 2i32,
}
#[cfg(feature = "LiteNetLib+ShutdownResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::ShutdownResult => "LiteNetLib"
    ."ShutdownResult"
);
