#[cfg(feature = "LiteNetLib+NatAddressType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NatAddressType {
    External = 1i32,
    Internal = 0i32,
}
#[cfg(feature = "LiteNetLib+NatAddressType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NatAddressType => "LiteNetLib"
    ."NatAddressType"
);
