#[cfg(feature = "System+Net+Configuration+UnicodeEncodingConformance")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnicodeEncodingConformance {
    Auto = 0i32,
    Compat = 2i32,
    Strict = 1i32,
}
#[cfg(feature = "System+Net+Configuration+UnicodeEncodingConformance")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Configuration::UnicodeEncodingConformance =>
    "System.Net.Configuration"."UnicodeEncodingConformance"
);
