#[cfg(feature = "Newtonsoft+Json+StringEscapeHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StringEscapeHandling {
    #[default]
    Default = 0i32,
    EscapeHtml = 2i32,
    EscapeNonAscii = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+StringEscapeHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::StringEscapeHandling =>
    "Newtonsoft.Json"."StringEscapeHandling"
);
