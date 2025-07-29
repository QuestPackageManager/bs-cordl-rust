#[cfg(feature = "cordl_class_System+Net+NetworkInformation+NetworkInterfaceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetworkInterfaceType {
    #[default]
    AsymmetricDsl = 94i32,
    Atm = 37i32,
    BasicIsdn = 20i32,
    Ethernet = 6i32,
    Ethernet3Megabit = 26i32,
    FastEthernetFx = 69i32,
    FastEthernetT = 62i32,
    Fddi = 15i32,
    GenericModem = 48i32,
    GigabitEthernet = 117i32,
    HighPerformanceSerialBus = 144i32,
    IPOverAtm = 114i32,
    Isdn = 63i32,
    Loopback = 24i32,
    MultiRateSymmetricDsl = 143i32,
    Ppp = 23i32,
    PrimaryIsdn = 21i32,
    RateAdaptDsl = 95i32,
    Slip = 28i32,
    SymmetricDsl = 96i32,
    TokenRing = 9i32,
    Tunnel = 131i32,
    Unknown = 1i32,
    VeryHighSpeedDsl = 97i32,
    Wireless80211 = 71i32,
    Wman = 237i32,
    Wwanpp = 243i32,
    Wwanpp2 = 244i32,
}
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+NetworkInterfaceType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::NetworkInformation::NetworkInterfaceType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net.NetworkInformation";
    const CLASS_NAME: &'static str = "NetworkInterfaceType";
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
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+NetworkInterfaceType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::NetworkInformation::NetworkInterfaceType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+NetworkInterfaceType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::NetworkInformation::NetworkInterfaceType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+NetworkInterfaceType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::NetworkInformation::NetworkInterfaceType {
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
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+NetworkInterfaceType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::NetworkInformation::NetworkInterfaceType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
