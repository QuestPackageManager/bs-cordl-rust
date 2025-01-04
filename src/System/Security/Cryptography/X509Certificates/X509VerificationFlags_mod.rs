#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509VerificationFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X509VerificationFlags {
    #[default]
    AllFlags = 4095i32,
    AllowUnknownCertificateAuthority = 16i32,
    IgnoreCertificateAuthorityRevocationUnknown = 1024i32,
    IgnoreCtlNotTimeValid = 2i32,
    IgnoreCtlSignerRevocationUnknown = 512i32,
    IgnoreEndRevocationUnknown = 256i32,
    IgnoreInvalidBasicConstraints = 8i32,
    IgnoreInvalidName = 64i32,
    IgnoreInvalidPolicy = 128i32,
    IgnoreNotTimeNested = 4i32,
    IgnoreNotTimeValid = 1i32,
    IgnoreRootRevocationUnknown = 2048i32,
    IgnoreWrongUsage = 32i32,
    NoFlag = 0i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509VerificationFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509VerificationFlags =>
    "System.Security.Cryptography.X509Certificates"."X509VerificationFlags"
);
