#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ifreq_mtu {
    padding: quest_hook::libil2cpp::ValueTypePadding<20usize>,
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixStructs::ifreq_mtu =>
    "System.Net.NetworkInformation.AixStructs"."ifreq_mtu"
);
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
impl crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu {
    #[cfg(
        feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
    )]
    pub type _ifr_name_e__FixedBuffer = crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer;
}
#[cfg(
    feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ifreq_mtu__ifr_name_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer =>
    "System.Net.NetworkInformation.AixStructs"."ifreq_mtu/<ifr_name>e__FixedBuffer"
);
#[cfg(
    feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
impl crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer {}
