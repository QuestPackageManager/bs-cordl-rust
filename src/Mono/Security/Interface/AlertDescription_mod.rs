#[cfg(feature = "Mono+Security+Interface+AlertDescription")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertDescription {
    #[default]
    AccessDenied = 49u8,
    BadCertificate = 42u8,
    BadRecordMAC = 20u8,
    CertificateExpired = 45u8,
    CertificateRevoked = 44u8,
    CertificateUnknown = 46u8,
    CloseNotify = 0u8,
    DecodeError = 50u8,
    DecompressionFailure = 30u8,
    DecryptError = 51u8,
    DecryptionFailed_RESERVED = 21u8,
    ExportRestriction = 60u8,
    HandshakeFailure = 40u8,
    IlegalParameter = 47u8,
    InsuficientSecurity = 71u8,
    InternalError = 80u8,
    NoCertificate_RESERVED = 41u8,
    NoRenegotiation = 100u8,
    ProtocolVersion = 70u8,
    RecordOverflow = 22u8,
    UnexpectedMessage = 10u8,
    UnknownCA = 48u8,
    UnsupportedCertificate = 43u8,
    UnsupportedExtension = 110u8,
    UserCancelled = 90u8,
}
#[cfg(feature = "Mono+Security+Interface+AlertDescription")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Interface::AlertDescription =>
    "Mono.Security.Interface"."AlertDescription"
);
