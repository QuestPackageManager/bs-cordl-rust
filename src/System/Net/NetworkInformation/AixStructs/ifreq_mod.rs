#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifreq")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ifreq {
    padding: [u8; 18usize],
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifreq")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixStructs::ifreq =>
    "System.Net.NetworkInformation.AixStructs"."ifreq"
);
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifreq")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::AixStructs::ifreq {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifreq")]
impl crate::System::Net::NetworkInformation::AixStructs::ifreq {
    #[cfg(
        feature = "System+Net+NetworkInformation+AixStructs+ifreq+_ifr_name_e__FixedBuffer"
    )]
    pub type _ifr_name_e__FixedBuffer = crate::System::Net::NetworkInformation::AixStructs::ifreq__ifr_name_e__FixedBuffer;
}
#[cfg(
    feature = "System+Net+NetworkInformation+AixStructs+ifreq+_ifr_name_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ifreq__ifr_name_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "System+Net+NetworkInformation+AixStructs+ifreq+_ifr_name_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixStructs::ifreq__ifr_name_e__FixedBuffer =>
    "System.Net.NetworkInformation.AixStructs"."ifreq/<ifr_name>e__FixedBuffer"
);
#[cfg(
    feature = "System+Net+NetworkInformation+AixStructs+ifreq+_ifr_name_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::AixStructs::ifreq__ifr_name_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "System+Net+NetworkInformation+AixStructs+ifreq+_ifr_name_e__FixedBuffer"
)]
impl crate::System::Net::NetworkInformation::AixStructs::ifreq__ifr_name_e__FixedBuffer {}
