#[cfg(feature = "System+Security+Cryptography+OidGroup")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OidGroup {
    #[default]
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
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::OidGroup {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography";
    const CLASS_NAME: &'static str = "OidGroup";
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
#[cfg(feature = "System+Security+Cryptography+OidGroup")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Security::Cryptography::OidGroup {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Security+Cryptography+OidGroup")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Security::Cryptography::OidGroup {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Security+Cryptography+OidGroup")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Security::Cryptography::OidGroup {
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
#[cfg(feature = "System+Security+Cryptography+OidGroup")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Security::Cryptography::OidGroup {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
