#[cfg(feature = "System+Net+Sockets+ProtocolType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProtocolType {
    #[default]
    Ggp = 3i32,
    IP = 0i32,
    IPSecAuthenticationHeader = 51i32,
    IPSecEncapsulatingSecurityPayload = 50i32,
    IPv4 = 4i32,
    IPv6 = 41i32,
    IPv6DestinationOptions = 60i32,
    IPv6FragmentHeader = 44i32,
    IPv6NoNextHeader = 59i32,
    IPv6RoutingHeader = 43i32,
    Icmp = 1i32,
    IcmpV6 = 58i32,
    Idp = 22i32,
    Igmp = 2i32,
    Ipx = 1000i32,
    ND = 77i32,
    Pup = 12i32,
    Raw = 255i32,
    Spx = 1256i32,
    SpxII = 1257i32,
    Tcp = 6i32,
    Udp = 17i32,
    Unknown = -1i32,
}
#[cfg(feature = "System+Net+Sockets+ProtocolType")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::Sockets::ProtocolType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net.Sockets";
    const CLASS_NAME: &'static str = "ProtocolType";
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
#[cfg(feature = "System+Net+Sockets+ProtocolType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::Sockets::ProtocolType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+Sockets+ProtocolType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::Sockets::ProtocolType {
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
#[cfg(feature = "System+Net+Sockets+ProtocolType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::Sockets::ProtocolType {
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
#[cfg(feature = "System+Net+Sockets+ProtocolType")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Net::Sockets::ProtocolType {
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
