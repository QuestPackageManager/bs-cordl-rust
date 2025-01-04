#[cfg(feature = "Newtonsoft+Json+FloatParseHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FloatParseHandling {
    #[default]
    Decimal = 1i32,
    Double = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+FloatParseHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::FloatParseHandling =>
    "Newtonsoft.Json"."FloatParseHandling"
);
