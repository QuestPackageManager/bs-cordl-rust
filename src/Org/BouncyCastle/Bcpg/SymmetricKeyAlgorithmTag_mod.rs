#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SymmetricKeyAlgorithmTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymmetricKeyAlgorithmTag {
    #[default]
    Aes128 = 7i32,
    Aes192 = 8i32,
    Aes256 = 9i32,
    Blowfish = 4i32,
    Camellia128 = 11i32,
    Camellia192 = 12i32,
    Camellia256 = 13i32,
    Cast5 = 3i32,
    Des = 6i32,
    Idea = 1i32,
    Null = 0i32,
    Safer = 5i32,
    TripleDes = 2i32,
    Twofish = 10i32,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SymmetricKeyAlgorithmTag")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg";
    const CLASS_NAME: &'static str = "SymmetricKeyAlgorithmTag";
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
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SymmetricKeyAlgorithmTag")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SymmetricKeyAlgorithmTag")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag {
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
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SymmetricKeyAlgorithmTag")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag {
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
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+SymmetricKeyAlgorithmTag")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag {
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
