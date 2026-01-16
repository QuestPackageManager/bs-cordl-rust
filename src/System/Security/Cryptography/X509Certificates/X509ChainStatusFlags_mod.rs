#[cfg(feature = "cordl_class_System+Security+Cryptography+X509Certificates+X509ChainStatusFlags")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum X509ChainStatusFlags {
    #[default]
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
#[cfg(feature = "cordl_class_System+Security+Cryptography+X509Certificates+X509ChainStatusFlags")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography.X509Certificates";
    const CLASS_NAME: &'static str = "X509ChainStatusFlags";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_System+Security+Cryptography+X509Certificates+X509ChainStatusFlags")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Security+Cryptography+X509Certificates+X509ChainStatusFlags")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags
{
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
#[cfg(feature = "cordl_class_System+Security+Cryptography+X509Certificates+X509ChainStatusFlags")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_System+Security+Cryptography+X509Certificates+X509ChainStatusFlags")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags
{
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
