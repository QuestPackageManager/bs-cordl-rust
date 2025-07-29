#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NtlmFlags {
    #[default]
    Negotiate128 = 536870912i32,
    Negotiate56 = -2147483648i32,
    NegotiateAlwaysSign = 32768i32,
    NegotiateDomainSupplied = 4096i32,
    NegotiateNtlm = 512i32,
    NegotiateNtlm2Key = 524288i32,
    NegotiateOem = 2i32,
    NegotiateUnicode = 1i32,
    NegotiateWorkstationSupplied = 8192i32,
    RequestTarget = 4i32,
}
#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmFlags")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Security::Protocol::Ntlm::NtlmFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Security.Protocol.Ntlm";
    const CLASS_NAME: &'static str = "NtlmFlags";
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
#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmFlags")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Security::Protocol::Ntlm::NtlmFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmFlags")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Security::Protocol::Ntlm::NtlmFlags {
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
#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmFlags")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Security::Protocol::Ntlm::NtlmFlags {
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
#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmFlags")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Security::Protocol::Ntlm::NtlmFlags {
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
