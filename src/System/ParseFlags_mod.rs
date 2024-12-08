#[cfg(feature = "System+ParseFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseFlags {
    CaptureOffset = 2048i32,
    HaveDate = 128i32,
    HaveDay = 4i32,
    HaveHour = 8i32,
    HaveMinute = 16i32,
    HaveMonth = 2i32,
    HaveSecond = 32i32,
    HaveTime = 64i32,
    HaveYear = 1i32,
    ParsedMonthName = 1024i32,
    Rfc1123Pattern = 8192i32,
    TimeZoneUsed = 256i32,
    TimeZoneUtc = 512i32,
    UtcSortPattern = 16384i32,
    YearDefault = 4096i32,
}
#[cfg(feature = "System+ParseFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ParseFlags => "System"."ParseFlags"
);
