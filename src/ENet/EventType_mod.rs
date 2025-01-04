#[cfg(feature = "ENet+EventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventType {
    #[default]
    Connect = 1i32,
    Disconnect = 2i32,
    None = 0i32,
    Receive = 3i32,
    Timeout = 4i32,
}
#[cfg(feature = "ENet+EventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::EventType => "ENet"."EventType"
);
