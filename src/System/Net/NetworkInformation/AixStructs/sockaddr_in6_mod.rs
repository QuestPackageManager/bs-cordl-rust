#[cfg(feature = "System+Net+NetworkInformation+AixStructs+sockaddr_in6")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct sockaddr_in6 {
    pub sin6_len: u8,
    pub sin6_family: u8,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: crate::System::Net::NetworkInformation::AixStructs::in6_addr,
    pub sin6_scope_id: u32,
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+sockaddr_in6")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixStructs::sockaddr_in6 =>
    "System.Net.NetworkInformation.AixStructs"."sockaddr_in6"
);
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+sockaddr_in6")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::AixStructs::sockaddr_in6 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+sockaddr_in6")]
impl crate::System::Net::NetworkInformation::AixStructs::sockaddr_in6 {}
