#[cfg(feature = "LiteNetLib+ConnectionRequestResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionRequestResult {
    #[default]
    Accept = 1i32,
    None = 0i32,
    Reject = 2i32,
    RejectForce = 3i32,
}
#[cfg(feature = "LiteNetLib+ConnectionRequestResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::ConnectionRequestResult =>
    "LiteNetLib"."ConnectionRequestResult"
);
