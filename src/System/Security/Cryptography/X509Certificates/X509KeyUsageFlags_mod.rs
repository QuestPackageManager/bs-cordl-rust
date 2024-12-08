#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X509KeyUsageFlags {
    CrlSign = 2i32,
    DataEncipherment = 16i32,
    DecipherOnly = 32768i32,
    DigitalSignature = 128i32,
    EncipherOnly = 1i32,
    KeyAgreement = 8i32,
    KeyCertSign = 4i32,
    KeyEncipherment = 32i32,
    NonRepudiation = 64i32,
    None = 0i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags =>
    "System.Security.Cryptography.X509Certificates"."X509KeyUsageFlags"
);
