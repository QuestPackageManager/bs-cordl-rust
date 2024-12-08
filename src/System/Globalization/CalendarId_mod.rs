#[cfg(feature = "System+Globalization+CalendarId")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarId {
    CHINESELUNISOLAR = 15u16,
    GREGORIAN = 1u16,
    GREGORIAN_ARABIC = 10u16,
    GREGORIAN_ME_FRENCH = 9u16,
    GREGORIAN_US = 2u16,
    GREGORIAN_XLIT_ENGLISH = 11u16,
    GREGORIAN_XLIT_FRENCH = 12u16,
    HEBREW = 8u16,
    HIJRI = 6u16,
    JAPAN = 3u16,
    JAPANESELUNISOLAR = 14u16,
    JULIAN = 13u16,
    KOREA = 5u16,
    KOREANLUNISOLAR = 20u16,
    LAST_CALENDAR = 23u16,
    LUNAR_ETO_CHN = 17u16,
    LUNAR_ETO_KOR = 18u16,
    LUNAR_ETO_ROKUYOU = 19u16,
    PERSIAN = 22u16,
    SAKA = 16u16,
    TAIWAN = 4u16,
    TAIWANLUNISOLAR = 21u16,
    THAI = 7u16,
    UNINITIALIZED_VALUE = 0u16,
}
#[cfg(feature = "System+Globalization+CalendarId")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::CalendarId =>
    "System.Globalization"."CalendarId"
);
