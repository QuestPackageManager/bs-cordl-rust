#[cfg(feature = "System+Net+NetworkInformation+AixStructs+in6_addr")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct in6_addr {
    pub u6_addr8: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+in6_addr")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixStructs::in6_addr =>
    "System.Net.NetworkInformation.AixStructs"."in6_addr"
);
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+in6_addr")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::AixStructs::in6_addr {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+in6_addr")]
impl crate::System::Net::NetworkInformation::AixStructs::in6_addr {}
