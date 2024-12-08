#[cfg(feature = "System+Globalization+HebrewNumberParsingState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HebrewNumberParsingState {
    ContinueParsing = 3i32,
    FoundEndOfHebrewNumber = 2i32,
    InvalidHebrewNumber = 0i32,
    NotHebrewDigit = 1i32,
}
#[cfg(feature = "System+Globalization+HebrewNumberParsingState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::HebrewNumberParsingState
    => "System.Globalization"."HebrewNumberParsingState"
);
