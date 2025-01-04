#[cfg(feature = "Newtonsoft+Json+DateParseHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DateParseHandling {
    #[default]
    DateTime = 1i32,
    DateTimeOffset = 2i32,
    None = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+DateParseHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::DateParseHandling =>
    "Newtonsoft.Json"."DateParseHandling"
);
