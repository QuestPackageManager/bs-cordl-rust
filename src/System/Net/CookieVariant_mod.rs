#[cfg(feature = "System+Net+CookieVariant")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookieVariant {
    Default = 2i32,
    Plain = 1i32,
    Rfc2965 = 3i32,
    Unknown = 0i32,
}
#[cfg(feature = "System+Net+CookieVariant")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CookieVariant => "System.Net"
    ."CookieVariant"
);
