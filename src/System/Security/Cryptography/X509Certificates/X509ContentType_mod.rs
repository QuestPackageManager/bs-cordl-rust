#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ContentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X509ContentType {
    #[default]
    Authenticode = 6i32,
    Cert = 1i32,
    Pfx = 3i32,
    Pkcs7 = 5i32,
    SerializedCert = 2i32,
    SerializedStore = 4i32,
    Unknown = 0i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ContentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509ContentType =>
    "System.Security.Cryptography.X509Certificates"."X509ContentType"
);
