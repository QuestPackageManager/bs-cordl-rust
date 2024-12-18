#[cfg(feature = "System+Globalization+FORMATFLAGS")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FORMATFLAGS {
    None = 0i32,
    UseDigitPrefixInTokens = 32i32,
    UseGenitiveMonth = 1i32,
    UseHebrewParsing = 8i32,
    UseLeapYearMonth = 2i32,
    UseSpacesInDayNames = 16i32,
    UseSpacesInMonthNames = 4i32,
}
#[cfg(feature = "System+Globalization+FORMATFLAGS")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::FORMATFLAGS =>
    "System.Globalization"."FORMATFLAGS"
);
