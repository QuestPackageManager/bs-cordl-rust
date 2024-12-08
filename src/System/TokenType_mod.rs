#[cfg(feature = "System+TokenType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Am = 3i32,
    DateWordToken = 10i32,
    DayOfWeekToken = 7i32,
    EndOfString = 6i32,
    EraToken = 9i32,
    HebrewNumber = 12i32,
    IgnorableSymbol = 15i32,
    JapaneseEraToken = 13i32,
    MonthToken = 5i32,
    NumberToken = 1i32,
    Pm = 4i32,
    RegularTokenMask = 255i32,
    SEP_Am = 1024i32,
    SEP_Date = 1536i32,
    SEP_DateOrOffset = 3840i32,
    SEP_DaySuff = 2560i32,
    SEP_End = 512i32,
    SEP_HourSuff = 2816i32,
    SEP_LocalTimeMark = 3584i32,
    SEP_MinuteSuff = 3072i32,
    SEP_MonthSuff = 2304i32,
    SEP_Pm = 1280i32,
    SEP_SecondSuff = 3328i32,
    SEP_Space = 768i32,
    SEP_Time = 1792i32,
    SEP_Unk = 256i32,
    SEP_YearSuff = 2048i32,
    SeparatorTokenMask = 65280i32,
    TEraToken = 14i32,
    TimeZoneToken = 8i32,
    UnknownToken = 11i32,
    YearNumberToken = 2i32,
}
#[cfg(feature = "System+TokenType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TokenType => "System"."TokenType"
);
