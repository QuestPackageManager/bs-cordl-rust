#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainStatusFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X509ChainStatusFlags {
    CtlNotSignatureValid = 262144i32,
    CtlNotTimeValid = 131072i32,
    CtlNotValidForUsage = 524288i32,
    Cyclic = 128i32,
    ExplicitDistrust = 67108864i32,
    HasExcludedNameConstraint = 32768i32,
    HasNotDefinedNameConstraint = 8192i32,
    HasNotPermittedNameConstraint = 16384i32,
    HasNotSupportedCriticalExtension = 134217728i32,
    HasNotSupportedNameConstraint = 4096i32,
    HasWeakSignature = 1048576i32,
    InvalidBasicConstraints = 1024i32,
    InvalidExtension = 256i32,
    InvalidNameConstraints = 2048i32,
    InvalidPolicyConstraints = 512i32,
    NoError = 0i32,
    NoIssuanceChainPolicy = 33554432i32,
    NotSignatureValid = 8i32,
    NotTimeNested = 2i32,
    NotTimeValid = 1i32,
    NotValidForUsage = 16i32,
    OfflineRevocation = 16777216i32,
    PartialChain = 65536i32,
    RevocationStatusUnknown = 64i32,
    Revoked = 4i32,
    UntrustedRoot = 32i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainStatusFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags =>
    "System.Security.Cryptography.X509Certificates"."X509ChainStatusFlags"
);
