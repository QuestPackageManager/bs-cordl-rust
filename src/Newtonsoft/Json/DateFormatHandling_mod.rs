#[cfg(feature = "Newtonsoft+Json+DateFormatHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DateFormatHandling {
    #[default]
    IsoDateFormat = 0i32,
    MicrosoftDateFormat = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+DateFormatHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::DateFormatHandling =>
    "Newtonsoft.Json"."DateFormatHandling"
);
