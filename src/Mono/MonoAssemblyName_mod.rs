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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::MonoAssemblyName => "Mono"
    ."MonoAssemblyName"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::MonoAssemblyName__public_key_token_e__FixedBuffer => "Mono"
    ."MonoAssemblyName/<public_key_token>e__FixedBuffer"
);
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
