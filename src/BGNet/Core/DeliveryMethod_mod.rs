#[cfg(feature = "BGNet+Core+DeliveryMethod")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryMethod {
    ReliableOrdered = 1u8,
    Unreliable = 0u8,
}
#[cfg(feature = "BGNet+Core+DeliveryMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BGNet::Core::DeliveryMethod => "BGNet.Core"
    ."DeliveryMethod"
);
