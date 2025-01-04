#[cfg(feature = "System+UriHostNameType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UriHostNameType {
    #[default]
    Basic = 1i32,
    Dns = 2i32,
    IPv4 = 3i32,
    IPv6 = 4i32,
    Unknown = 0i32,
}
#[cfg(feature = "System+UriHostNameType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UriHostNameType => "System"
    ."UriHostNameType"
);
