#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509RevocationFlag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X509RevocationFlag {
    #[default]
    EndCertificateOnly = 0i32,
    EntireChain = 1i32,
    ExcludeRoot = 2i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509RevocationFlag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509RevocationFlag =>
    "System.Security.Cryptography.X509Certificates"."X509RevocationFlag"
);
