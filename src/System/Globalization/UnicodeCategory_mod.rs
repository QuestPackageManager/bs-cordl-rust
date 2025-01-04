#[cfg(feature = "System+Globalization+UnicodeCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnicodeCategory {
    #[default]
    ClosePunctuation = 21i32,
    ConnectorPunctuation = 18i32,
    Control = 14i32,
    CurrencySymbol = 26i32,
    DashPunctuation = 19i32,
    DecimalDigitNumber = 8i32,
    EnclosingMark = 7i32,
    FinalQuotePunctuation = 23i32,
    Format = 15i32,
    InitialQuotePunctuation = 22i32,
    LetterNumber = 9i32,
    LineSeparator = 12i32,
    LowercaseLetter = 1i32,
    MathSymbol = 25i32,
    ModifierLetter = 3i32,
    ModifierSymbol = 27i32,
    NonSpacingMark = 5i32,
    OpenPunctuation = 20i32,
    OtherLetter = 4i32,
    OtherNotAssigned = 29i32,
    OtherNumber = 10i32,
    OtherPunctuation = 24i32,
    OtherSymbol = 28i32,
    ParagraphSeparator = 13i32,
    PrivateUse = 17i32,
    SpaceSeparator = 11i32,
    SpacingCombiningMark = 6i32,
    Surrogate = 16i32,
    TitlecaseLetter = 2i32,
    UppercaseLetter = 0i32,
}
#[cfg(feature = "System+Globalization+UnicodeCategory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::UnicodeCategory =>
    "System.Globalization"."UnicodeCategory"
);
