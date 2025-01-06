#[cfg(feature = "System+Net+NetworkInformation+sockaddr_ll")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct sockaddr_ll {
    pub sll_family: u16,
    pub sll_protocol: u16,
    pub sll_ifindex: i32,
    pub sll_hatype: u16,
    pub sll_pkttype: u8,
    pub sll_halen: u8,
    pub sll_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "System+Net+NetworkInformation+sockaddr_ll")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NetworkInformation::sockaddr_ll =>
    "System.Net.NetworkInformation"."sockaddr_ll"
);
#[cfg(feature = "System+Net+NetworkInformation+sockaddr_ll")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::sockaddr_ll {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+sockaddr_ll")]
impl crate::System::Net::NetworkInformation::sockaddr_ll {}
