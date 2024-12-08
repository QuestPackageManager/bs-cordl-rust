#[cfg(feature = "System+Security+Cryptography+RSAParameters")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RSAParameters {
    pub Exponent: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Modulus: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub P: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Q: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub DP: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub DQ: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub InverseQ: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub D: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "System+Security+Cryptography+RSAParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::RSAParameters =>
    "System.Security.Cryptography"."RSAParameters"
);
#[cfg(feature = "System+Security+Cryptography+RSAParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Security::Cryptography::RSAParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Security+Cryptography+RSAParameters")]
impl crate::System::Security::Cryptography::RSAParameters {}
