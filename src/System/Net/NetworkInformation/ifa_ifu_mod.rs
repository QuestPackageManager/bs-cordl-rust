#[cfg(feature = "System+Net+NetworkInformation+ifa_ifu")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ifa_ifu {
    padding: [u8; 8usize],
}
#[cfg(feature = "System+Net+NetworkInformation+ifa_ifu")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NetworkInformation::ifa_ifu =>
    "System.Net.NetworkInformation"."ifa_ifu"
);
#[cfg(feature = "System+Net+NetworkInformation+ifa_ifu")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::ifa_ifu {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+ifa_ifu")]
impl crate::System::Net::NetworkInformation::ifa_ifu {}
