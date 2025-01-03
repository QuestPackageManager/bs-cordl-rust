#[cfg(feature = "ENet+SslMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SslMode {
    Client = 2i32,
    None = 0i32,
    Server = 1i32,
}
#[cfg(feature = "ENet+SslMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::SslMode => "ENet"."SslMode"
);
