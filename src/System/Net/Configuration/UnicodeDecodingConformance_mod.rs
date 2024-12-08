#[cfg(feature = "System+Net+Configuration+UnicodeDecodingConformance")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnicodeDecodingConformance {
    Auto = 0i32,
    Compat = 2i32,
    Loose = 3i32,
    Strict = 1i32,
}
#[cfg(feature = "System+Net+Configuration+UnicodeDecodingConformance")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Configuration::UnicodeDecodingConformance =>
    "System.Net.Configuration"."UnicodeDecodingConformance"
);
