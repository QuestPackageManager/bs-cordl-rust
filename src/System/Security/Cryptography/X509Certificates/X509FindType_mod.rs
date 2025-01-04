#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509FindType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X509FindType {
    #[default]
    FindByApplicationPolicy = 10i32,
    FindByCertificatePolicy = 11i32,
    FindByExtension = 12i32,
    FindByIssuerDistinguishedName = 4i32,
    FindByIssuerName = 3i32,
    FindByKeyUsage = 13i32,
    FindBySerialNumber = 5i32,
    FindBySubjectDistinguishedName = 2i32,
    FindBySubjectKeyIdentifier = 14i32,
    FindBySubjectName = 1i32,
    FindByTemplateName = 9i32,
    FindByThumbprint = 0i32,
    FindByTimeExpired = 8i32,
    FindByTimeNotYetValid = 7i32,
    FindByTimeValid = 6i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509FindType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509FindType =>
    "System.Security.Cryptography.X509Certificates"."X509FindType"
);
