#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedNameFlags"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X500DistinguishedNameFlags {
    #[default]
    DoNotUsePlusSign = 32i32,
    DoNotUseQuotes = 64i32,
    ForceUTF8Encoding = 16384i32,
    None = 0i32,
    Reversed = 1i32,
    UseCommas = 128i32,
    UseNewLines = 256i32,
    UseSemicolons = 16i32,
    UseT61Encoding = 8192i32,
    UseUTF8Encoding = 4096i32,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedNameFlags"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::X509Certificates::X500DistinguishedNameFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography.X509Certificates";
    const CLASS_NAME: &'static str = "X500DistinguishedNameFlags";
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
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedNameFlags"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Security::Cryptography::X509Certificates::X500DistinguishedNameFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedNameFlags"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Security::Cryptography::X509Certificates::X500DistinguishedNameFlags {
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
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedNameFlags"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Security::Cryptography::X509Certificates::X500DistinguishedNameFlags {
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
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedNameFlags"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Security::Cryptography::X509Certificates::X500DistinguishedNameFlags {
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
