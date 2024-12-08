#[cfg(feature = "Newtonsoft+Json+Utilities+ParseResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseResult {
    Invalid = 3i32,
    None = 0i32,
    Overflow = 2i32,
    Success = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ParseResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::ParseResult =>
    "Newtonsoft.Json.Utilities"."ParseResult"
);
