#[cfg(feature = "LiteNetLib+NetLogLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetLogLevel {
    Error = 1i32,
    Info = 3i32,
    Trace = 2i32,
    Warning = 0i32,
}
#[cfg(feature = "LiteNetLib+NetLogLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetLogLevel => "LiteNetLib"
    ."NetLogLevel"
);
