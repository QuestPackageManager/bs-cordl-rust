#[cfg(feature = "System+ParseFailureKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParseFailureKind {
    #[default]
    ArgumentNull = 1i32,
    Format = 2i32,
    FormatBadDateTimeCalendar = 7i32,
    FormatWithFormatSpecifier = 5i32,
    FormatWithOriginalDateTime = 4i32,
    FormatWithOriginalDateTimeAndParameter = 6i32,
    FormatWithParameter = 3i32,
    None = 0i32,
}
#[cfg(feature = "System+ParseFailureKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ParseFailureKind => "System"
    ."ParseFailureKind"
);
