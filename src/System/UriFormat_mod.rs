#[cfg(feature = "System+UriFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UriFormat {
    #[default]
    SafeUnescaped = 3i32,
    Unescaped = 2i32,
    UriEscaped = 1i32,
}
#[cfg(feature = "System+UriFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UriFormat => "System"."UriFormat"
);
