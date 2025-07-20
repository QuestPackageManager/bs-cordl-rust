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
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Security::Interface::AlertDescription {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Security.Interface";
    const CLASS_NAME: &'static str = "AlertDescription";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Mono+Security+Interface+AlertDescription")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Security::Interface::AlertDescription {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Security+Interface+AlertDescription")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Security::Interface::AlertDescription {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "Mono+Security+Interface+AlertDescription")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Security::Interface::AlertDescription {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Mono+Security+Interface+AlertDescription")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Security::Interface::AlertDescription {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
