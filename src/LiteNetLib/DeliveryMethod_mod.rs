#[cfg(feature = "LiteNetLib+DeliveryMethod")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeliveryMethod {
    #[default]
    ReliableOrdered = 2u8,
    ReliableSequenced = 3u8,
    ReliableUnordered = 0u8,
    Sequenced = 1u8,
    Unreliable = 4u8,
}
#[cfg(feature = "LiteNetLib+DeliveryMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::DeliveryMethod => "LiteNetLib"
    ."DeliveryMethod"
);
