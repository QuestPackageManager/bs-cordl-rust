#[cfg(feature = "System+Globalization+MonthNameStyles")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthNameStyles {
    Genitive = 1i32,
    LeapYear = 2i32,
    Regular = 0i32,
}
#[cfg(feature = "System+Globalization+MonthNameStyles")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::MonthNameStyles =>
    "System.Globalization"."MonthNameStyles"
);
