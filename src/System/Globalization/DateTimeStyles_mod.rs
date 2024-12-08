#[cfg(feature = "System+Globalization+DateTimeStyles")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeStyles {
    AdjustToUniversal = 16i32,
    AllowInnerWhite = 4i32,
    AllowLeadingWhite = 1i32,
    AllowTrailingWhite = 2i32,
    AllowWhiteSpaces = 7i32,
    AssumeLocal = 32i32,
    AssumeUniversal = 64i32,
    NoCurrentDateDefault = 8i32,
    None = 0i32,
    RoundtripKind = 128i32,
}
#[cfg(feature = "System+Globalization+DateTimeStyles")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::DateTimeStyles =>
    "System.Globalization"."DateTimeStyles"
);
