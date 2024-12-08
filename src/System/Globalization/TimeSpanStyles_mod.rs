#[cfg(feature = "System+Globalization+TimeSpanStyles")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSpanStyles {
    AssumeNegative = 1i32,
    None = 0i32,
}
#[cfg(feature = "System+Globalization+TimeSpanStyles")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::TimeSpanStyles =>
    "System.Globalization"."TimeSpanStyles"
);
