#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericStructType")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct __Il2CppFullySharedGenericStructType {}
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericStructType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericStructType =>
    "Unity.IL2CPP.Metadata"."__Il2CppFullySharedGenericStructType"
);
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericStructType")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericStructType {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+IL2CPP+Metadata+__Il2CppFullySharedGenericStructType")]
impl crate::Unity::IL2CPP::Metadata::__Il2CppFullySharedGenericStructType {}
