#[cfg(feature = "Mono+MonoAssemblyName")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonoAssemblyName {
    pub name: crate::System::IntPtr,
    pub culture: crate::System::IntPtr,
    pub hash_value: crate::System::IntPtr,
    pub public_key: crate::System::IntPtr,
    pub public_key_token: crate::Mono::MonoAssemblyName__public_key_token_e__FixedBuffer,
    pub hash_alg: u32,
    pub hash_len: u32,
    pub flags: u32,
    pub major: u16,
    pub minor: u16,
    pub build: u16,
    pub revision: u16,
    pub arch: u16,
}
#[cfg(feature = "Mono+MonoAssemblyName")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::MonoAssemblyName {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono";
    const CLASS_NAME: &'static str = "MonoAssemblyName";
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
#[cfg(feature = "Mono+MonoAssemblyName")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Mono::MonoAssemblyName {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+MonoAssemblyName")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Mono::MonoAssemblyName {
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
#[cfg(feature = "Mono+MonoAssemblyName")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Mono::MonoAssemblyName {
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
#[cfg(feature = "Mono+MonoAssemblyName")]
unsafe impl quest_hook::libil2cpp::Return for crate::Mono::MonoAssemblyName {
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
#[cfg(feature = "Mono+MonoAssemblyName")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Mono::MonoAssemblyName {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+MonoAssemblyName")]
impl crate::Mono::MonoAssemblyName {
    #[cfg(feature = "Mono+MonoAssemblyName+_public_key_token_e__FixedBuffer")]
    pub type _public_key_token_e__FixedBuffer = crate::Mono::MonoAssemblyName__public_key_token_e__FixedBuffer;
}
#[cfg(feature = "Mono+MonoAssemblyName+_public_key_token_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonoAssemblyName__public_key_token_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(feature = "Mono+MonoAssemblyName+_public_key_token_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::MonoAssemblyName__public_key_token_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono";
    const CLASS_NAME: &'static str = "MonoAssemblyName/<public_key_token>e__FixedBuffer";
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
#[cfg(feature = "Mono+MonoAssemblyName+_public_key_token_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::MonoAssemblyName__public_key_token_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+MonoAssemblyName+_public_key_token_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::MonoAssemblyName__public_key_token_e__FixedBuffer {
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
#[cfg(feature = "Mono+MonoAssemblyName+_public_key_token_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::MonoAssemblyName__public_key_token_e__FixedBuffer {
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
#[cfg(feature = "Mono+MonoAssemblyName+_public_key_token_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::MonoAssemblyName__public_key_token_e__FixedBuffer {
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
#[cfg(feature = "Mono+MonoAssemblyName+_public_key_token_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::MonoAssemblyName__public_key_token_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+MonoAssemblyName+_public_key_token_e__FixedBuffer")]
impl crate::Mono::MonoAssemblyName__public_key_token_e__FixedBuffer {}
