#[cfg(feature = "Newtonsoft+Json+FloatFormatHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatFormatHandling {
    DefaultValue = 2i32,
    String = 0i32,
    Symbol = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+FloatFormatHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::FloatFormatHandling =>
    "Newtonsoft.Json"."FloatFormatHandling"
);
