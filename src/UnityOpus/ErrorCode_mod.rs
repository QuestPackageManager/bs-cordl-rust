#[cfg(feature = "UnityOpus+ErrorCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErrorCode {
    #[default]
    AllocFail = -7i32,
    BadArg = -1i32,
    BufferTooSmall = -2i32,
    InternalError = -3i32,
    InvalidPacket = -4i32,
    InvalidState = -6i32,
    OK = 0i32,
    Unimplemented = -5i32,
}
#[cfg(feature = "UnityOpus+ErrorCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityOpus::ErrorCode => "UnityOpus"."ErrorCode"
);
