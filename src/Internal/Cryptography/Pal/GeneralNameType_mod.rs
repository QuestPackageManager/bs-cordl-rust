#[cfg(feature = "cordl_class_Internal+Cryptography+Pal+GeneralNameType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GeneralNameType {
    #[default]
    DirectoryName = 4i32,
    DnsName = 2i32,
    EdiPartyName = 5i32,
    Email = 1i32,
    IPAddress = 7i32,
    OtherName = 0i32,
    RegisteredId = 8i32,
    UniformResourceIdentifier = 6i32,
    X400Address = 3i32,
}
#[cfg(feature = "cordl_class_Internal+Cryptography+Pal+GeneralNameType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Internal::Cryptography::Pal::GeneralNameType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Internal.Cryptography.Pal";
    const CLASS_NAME: &'static str = "GeneralNameType";
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
#[cfg(feature = "cordl_class_Internal+Cryptography+Pal+GeneralNameType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Internal::Cryptography::Pal::GeneralNameType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Internal+Cryptography+Pal+GeneralNameType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Internal::Cryptography::Pal::GeneralNameType {
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
#[cfg(feature = "cordl_class_Internal+Cryptography+Pal+GeneralNameType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Internal::Cryptography::Pal::GeneralNameType {
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
#[cfg(feature = "cordl_class_Internal+Cryptography+Pal+GeneralNameType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Internal::Cryptography::Pal::GeneralNameType {
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
