#[cfg(feature = "System+Net+NetworkInformation+MacOsStructs+sockaddr_in")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct sockaddr_in {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: u32,
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsStructs+sockaddr_in")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::MacOsStructs::sockaddr_in =>
    "System.Net.NetworkInformation.MacOsStructs"."sockaddr_in"
);
#[cfg(feature = "System+Net+NetworkInformation+MacOsStructs+sockaddr_in")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::MacOsStructs::sockaddr_in {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsStructs+sockaddr_in")]
impl crate::System::Net::NetworkInformation::MacOsStructs::sockaddr_in {}
