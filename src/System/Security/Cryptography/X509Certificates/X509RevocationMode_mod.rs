#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509RevocationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X509RevocationMode {
    #[default]
    NoCheck = 0i32,
    Offline = 2i32,
    Online = 1i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509RevocationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509RevocationMode =>
    "System.Security.Cryptography.X509Certificates"."X509RevocationMode"
);
