#[cfg(feature = "PacketOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PacketOption {
    #[default]
    Encrypted = 1i32,
    None = 0i32,
    OnlyFirstDegreeConnections = 2i32,
}
#[cfg(feature = "PacketOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PacketOption => ""
    ."PacketOption"
);
