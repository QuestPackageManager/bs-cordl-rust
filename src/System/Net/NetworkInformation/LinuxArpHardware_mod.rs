#[cfg(feature = "System+Net+NetworkInformation+LinuxArpHardware")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinuxArpHardware {
    #[default]
    ATM = 19i32,
    CSLIP = 257i32,
    CSLIP6 = 259i32,
    EETHER = 2i32,
    ETHER = 1i32,
    FDDI = 774i32,
    IP6GRE = 823i32,
    IPDDP = 777i32,
    IPGRE = 778i32,
    LOOPBACK = 772i32,
    PPP = 512i32,
    PRONET = 4i32,
    SIT = 776i32,
    SLIP = 256i32,
    SLIP6 = 258i32,
    TUNNEL = 768i32,
    TUNNEL6 = 769i32,
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxArpHardware")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::NetworkInformation::LinuxArpHardware {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net.NetworkInformation";
    const CLASS_NAME: &'static str = "LinuxArpHardware";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxArpHardware")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::NetworkInformation::LinuxArpHardware {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxArpHardware")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::NetworkInformation::LinuxArpHardware {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxArpHardware")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::NetworkInformation::LinuxArpHardware {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxArpHardware")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::NetworkInformation::LinuxArpHardware {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
