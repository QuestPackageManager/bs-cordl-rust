#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SignatureSubpacketTag")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum SignatureSubpacketTag {
    #[cfg_attr(feature = "derive_Default", default)]
    CreationTime = 2i32,
    EmbeddedSignature = 32i32,
    ExpireTime = 3i32,
    Exportable = 4i32,
    Features = 30i32,
    IssuerKeyId = 16i32,
    KeyExpireTime = 9i32,
    KeyFlags = 27i32,
    KeyServerPreferences = 23i32,
    NotationData = 20i32,
    Placeholder = 10i32,
    PolicyUrl = 26i32,
    PreferredCompressionAlgorithms = 22i32,
    PreferredHashAlgorithms = 21i32,
    PreferredKeyServer = 24i32,
    PreferredSymmetricAlgorithms = 11i32,
    PrimaryUserId = 25i32,
    RegExp = 6i32,
    Revocable = 7i32,
    RevocationKey = 12i32,
    RevocationReason = 29i32,
    SignatureTarget = 31i32,
    SignerUserId = 28i32,
    TrustSig = 5i32,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SignatureSubpacketTag")]
unsafe impl quest_hook::libil2cpp::Type for crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg";
    const CLASS_NAME: &'static str = "SignatureSubpacketTag";
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
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SignatureSubpacketTag")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SignatureSubpacketTag")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag
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
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SignatureSubpacketTag")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag
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
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SignatureSubpacketTag")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag
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
