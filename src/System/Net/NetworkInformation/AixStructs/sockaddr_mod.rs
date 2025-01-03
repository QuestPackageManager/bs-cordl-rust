#[cfg(feature = "System+Net+NetworkInformation+AixStructs+sockaddr")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct sockaddr {
    pub sa_len: u8,
    pub sa_family: u8,
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+sockaddr")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixStructs::sockaddr =>
    "System.Net.NetworkInformation.AixStructs"."sockaddr"
);
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+sockaddr")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::AixStructs::sockaddr {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+sockaddr")]
impl crate::System::Net::NetworkInformation::AixStructs::sockaddr {}
