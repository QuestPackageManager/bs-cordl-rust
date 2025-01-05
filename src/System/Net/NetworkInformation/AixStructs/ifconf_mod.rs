#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifconf")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ifconf {
    padding: quest_hook::libil2cpp::ValueTypePadding<16usize>,
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifconf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixStructs::ifconf =>
    "System.Net.NetworkInformation.AixStructs"."ifconf"
);
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifconf")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::AixStructs::ifconf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifconf")]
impl crate::System::Net::NetworkInformation::AixStructs::ifconf {}
