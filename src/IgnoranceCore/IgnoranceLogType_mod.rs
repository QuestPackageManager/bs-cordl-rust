#[cfg(feature = "IgnoranceCore+IgnoranceLogType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IgnoranceLogType {
    Quiet = 0i32,
    Standard = 1i32,
    Verbose = 2i32,
}
#[cfg(feature = "IgnoranceCore+IgnoranceLogType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceLogType =>
    "IgnoranceCore"."IgnoranceLogType"
);
