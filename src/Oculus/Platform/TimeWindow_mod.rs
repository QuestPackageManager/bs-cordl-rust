#[cfg(feature = "Oculus+Platform+TimeWindow")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeWindow {
    NinetyDays = 5i32,
    OneDay = 2i32,
    OneHour = 1i32,
    OneWeek = 3i32,
    ThirtyDays = 4i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+TimeWindow")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::TimeWindow =>
    "Oculus.Platform"."TimeWindow"
);
