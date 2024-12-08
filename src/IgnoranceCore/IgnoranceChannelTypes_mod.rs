#[cfg(feature = "IgnoranceCore+IgnoranceChannelTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IgnoranceChannelTypes {
    Reliable = 1i32,
    ReliableUnsequenced = 3i32,
    Unreliable = 2i32,
    UnreliableFragmented = 8i32,
    UnreliableSequenced = 0i32,
    Unthrottled = 32i32,
}
#[cfg(feature = "IgnoranceCore+IgnoranceChannelTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceChannelTypes =>
    "IgnoranceCore"."IgnoranceChannelTypes"
);
