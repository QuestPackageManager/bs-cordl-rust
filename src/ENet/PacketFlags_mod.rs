#[cfg(feature = "ENet+PacketFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PacketFlags {
    #[default]
    Instant = 16i32,
    NoAllocate = 4i32,
    None = 0i32,
    Reliable = 1i32,
    Sent = 256i32,
    UnreliableFragmented = 8i32,
    Unsequenced = 2i32,
    Unthrottled = 32i32,
}
#[cfg(feature = "ENet+PacketFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::PacketFlags => "ENet"."PacketFlags"
);
