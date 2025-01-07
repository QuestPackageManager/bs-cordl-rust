#[cfg(feature = "Mono+Security+Interface+TlsProtocols")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TlsProtocols {
    #[default]
    ClientMask = 2688i32,
    ServerMask = 1344i32,
    Tls10 = 192i32,
    Tls10Client = 128i32,
    Tls10Server = 64i32,
    Tls11 = 768i32,
    Tls11Client = 512i32,
    Tls11Server = 256i32,
    Tls12 = 3072i32,
    Tls12Client = 2048i32,
    Tls12Server = 1024i32,
    Zero = 0i32,
}
#[cfg(feature = "Mono+Security+Interface+TlsProtocols")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Security::Interface::TlsProtocols {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Security.Interface";
    const CLASS_NAME: &'static str = "TlsProtocols";
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
#[cfg(feature = "Mono+Security+Interface+TlsProtocols")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Security::Interface::TlsProtocols {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Security+Interface+TlsProtocols")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Security::Interface::TlsProtocols {
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
#[cfg(feature = "Mono+Security+Interface+TlsProtocols")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Security::Interface::TlsProtocols {
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
#[cfg(feature = "Mono+Security+Interface+TlsProtocols")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Security::Interface::TlsProtocols {
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
