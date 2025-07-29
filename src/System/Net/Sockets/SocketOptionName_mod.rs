#[cfg(feature = "cordl_class_System+Net+Sockets+SocketOptionName")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SocketOptionName {
    #[default]
    AcceptConnection = 2i32,
    AddMembership = 12i32,
    AddSourceMembership = 15i32,
    BlockSource = 17i32,
    Broadcast = 32i32,
    ChecksumCoverage = 20i32,
    Debug = 1i32,
    DontFragment = 14i32,
    DontLinger = -129i32,
    DontRoute = 16i32,
    DropMembership = 13i32,
    Error = 4103i32,
    ExclusiveAddressUse = -5i32,
    HopLimit = 21i32,
    IPProtectionLevel = 23i32,
    IPv6Only = 27i32,
    IpTimeToLive = 4i32,
    KeepAlive = 8i32,
    Linger = 128i32,
    MaxConnections = 2147483647i32,
    MulticastInterface = 9i32,
    MulticastLoopback = 11i32,
    MulticastTimeToLive = 10i32,
    OutOfBandInline = 256i32,
    PacketInformation = 19i32,
    ReceiveBuffer = 4098i32,
    ReceiveLowWater = 4100i32,
    ReceiveTimeout = 4102i32,
    ReuseUnicastPort = 12295i32,
    SendBuffer = 4097i32,
    SendLowWater = 4099i32,
    SendTimeout = 4101i32,
    Type = 4104i32,
    TypeOfService = 3i32,
    UnblockSource = 18i32,
    UpdateAcceptContext = 28683i32,
    UpdateConnectContext = 28688i32,
    UseLoopback = 64i32,
}
#[cfg(feature = "cordl_class_System+Net+Sockets+SocketOptionName")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Sockets::SocketOptionName {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net.Sockets";
    const CLASS_NAME: &'static str = "SocketOptionName";
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
#[cfg(feature = "cordl_class_System+Net+Sockets+SocketOptionName")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::Sockets::SocketOptionName {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Net+Sockets+SocketOptionName")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::Sockets::SocketOptionName {
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
#[cfg(feature = "cordl_class_System+Net+Sockets+SocketOptionName")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::Sockets::SocketOptionName {
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
#[cfg(feature = "cordl_class_System+Net+Sockets+SocketOptionName")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::Sockets::SocketOptionName {
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
