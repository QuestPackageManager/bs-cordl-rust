#[cfg(feature = "System+DayOfWeek")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    Friday = 5i32,
    Monday = 1i32,
    Saturday = 6i32,
    Sunday = 0i32,
    Thursday = 4i32,
    Tuesday = 2i32,
    Wednesday = 3i32,
}
#[cfg(feature = "System+DayOfWeek")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DayOfWeek => "System"."DayOfWeek"
);
