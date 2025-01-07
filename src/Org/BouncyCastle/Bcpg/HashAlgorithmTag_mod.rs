#[cfg(feature = "Org+BouncyCastle+Bcpg+HashAlgorithmTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HashAlgorithmTag {
    #[default]
    DoubleSha = 4i32,
    Haval5pass160 = 7i32,
    MD2 = 5i32,
    MD5 = 1i32,
    RipeMD160 = 3i32,
    Sha1 = 2i32,
    Sha224 = 11i32,
    Sha256 = 8i32,
    Sha384 = 9i32,
    Sha512 = 10i32,
    Tiger192 = 6i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+HashAlgorithmTag")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg";
    const CLASS_NAME: &'static str = "HashAlgorithmTag";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+HashAlgorithmTag")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+HashAlgorithmTag")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag {
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+HashAlgorithmTag")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag {
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+HashAlgorithmTag")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag {
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
