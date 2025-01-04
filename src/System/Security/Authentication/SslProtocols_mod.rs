#[cfg(feature = "System+Security+Authentication+SslProtocols")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SslProtocols {
    #[default]
    Default = 240i32,
    None = 0i32,
    Ssl2 = 12i32,
    Ssl3 = 48i32,
    Tls = 192i32,
    Tls11 = 768i32,
    Tls12 = 3072i32,
    Tls13 = 12288i32,
}
#[cfg(feature = "System+Security+Authentication+SslProtocols")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Authentication::SslProtocols
    => "System.Security.Authentication"."SslProtocols"
);
