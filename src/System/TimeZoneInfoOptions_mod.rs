#[cfg(feature = "System+TimeZoneInfoOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeZoneInfoOptions {
    NoThrowOnInvalidTime = 2i32,
    None = 1i32,
}
#[cfg(feature = "System+TimeZoneInfoOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TimeZoneInfoOptions => "System"
    ."TimeZoneInfoOptions"
);
