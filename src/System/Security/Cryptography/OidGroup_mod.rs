#[cfg(feature = "System+Security+Cryptography+OidGroup")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OidGroup {
    All = 0i32,
    Attribute = 5i32,
    EncryptionAlgorithm = 2i32,
    EnhancedKeyUsage = 7i32,
    ExtensionOrAttribute = 6i32,
    HashAlgorithm = 1i32,
    KeyDerivationFunction = 10i32,
    Policy = 8i32,
    PublicKeyAlgorithm = 3i32,
    SignatureAlgorithm = 4i32,
    Template = 9i32,
}
#[cfg(feature = "System+Security+Cryptography+OidGroup")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::OidGroup =>
    "System.Security.Cryptography"."OidGroup"
);
