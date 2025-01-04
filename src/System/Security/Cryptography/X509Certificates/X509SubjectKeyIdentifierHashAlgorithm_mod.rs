#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509SubjectKeyIdentifierHashAlgorithm"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X509SubjectKeyIdentifierHashAlgorithm {
    #[default]
    CapiSha1 = 2i32,
    Sha1 = 0i32,
    ShortSha1 = 1i32,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509SubjectKeyIdentifierHashAlgorithm"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierHashAlgorithm
    => "System.Security.Cryptography.X509Certificates"
    ."X509SubjectKeyIdentifierHashAlgorithm"
);
