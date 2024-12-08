#[cfg(feature = "Newtonsoft+Json+DateTimeZoneHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeZoneHandling {
    Local = 0i32,
    RoundtripKind = 3i32,
    Unspecified = 2i32,
    Utc = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+DateTimeZoneHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::DateTimeZoneHandling =>
    "Newtonsoft.Json"."DateTimeZoneHandling"
);
