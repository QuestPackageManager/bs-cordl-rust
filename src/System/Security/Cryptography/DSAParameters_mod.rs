#[cfg(feature = "System+Security+Cryptography+DSAParameters")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DSAParameters {
    pub P: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Q: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub G: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Y: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub J: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub X: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Counter: i32,
}
#[cfg(feature = "System+Security+Cryptography+DSAParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::DSAParameters =>
    "System.Security.Cryptography"."DSAParameters"
);
#[cfg(feature = "System+Security+Cryptography+DSAParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Security::Cryptography::DSAParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Security+Cryptography+DSAParameters")]
impl crate::System::Security::Cryptography::DSAParameters {}
