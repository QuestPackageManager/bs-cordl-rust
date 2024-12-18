#[cfg(feature = "Newtonsoft+Json+Utilities+ParserTimeZone")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserTimeZone {
    LocalEastOfUtc = 3i32,
    LocalWestOfUtc = 2i32,
    Unspecified = 0i32,
    Utc = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ParserTimeZone")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::ParserTimeZone =>
    "Newtonsoft.Json.Utilities"."ParserTimeZone"
);
