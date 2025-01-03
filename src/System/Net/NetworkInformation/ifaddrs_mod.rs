#[cfg(feature = "System+Net+NetworkInformation+ifaddrs")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ifaddrs {
    pub ifa_next: crate::System::IntPtr,
    pub ifa_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ifa_flags: u32,
    pub ifa_addr: crate::System::IntPtr,
    pub ifa_netmask: crate::System::IntPtr,
    pub ifa_ifu: crate::System::Net::NetworkInformation::ifa_ifu,
    pub ifa_data: crate::System::IntPtr,
}
#[cfg(feature = "System+Net+NetworkInformation+ifaddrs")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NetworkInformation::ifaddrs =>
    "System.Net.NetworkInformation"."ifaddrs"
);
#[cfg(feature = "System+Net+NetworkInformation+ifaddrs")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::ifaddrs {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+ifaddrs")]
impl crate::System::Net::NetworkInformation::ifaddrs {}
