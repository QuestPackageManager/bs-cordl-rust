#[cfg(feature = "LiteNetLib+LocalAddrType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalAddrType {
    All = 3i32,
    IPv4 = 1i32,
    IPv6 = 2i32,
}
#[cfg(feature = "LiteNetLib+LocalAddrType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::LocalAddrType => "LiteNetLib"
    ."LocalAddrType"
);
