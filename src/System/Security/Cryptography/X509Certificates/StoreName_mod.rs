#[cfg(feature = "System+Security+Cryptography+X509Certificates+StoreName")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StoreName {
    #[default]
    AddressBook = 1i32,
    AuthRoot = 2i32,
    CertificateAuthority = 3i32,
    Disallowed = 4i32,
    My = 5i32,
    Root = 6i32,
    TrustedPeople = 7i32,
    TrustedPublisher = 8i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+StoreName")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::StoreName =>
    "System.Security.Cryptography.X509Certificates"."StoreName"
);
