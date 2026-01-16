#[cfg(feature = "cordl_class_System+Net+Sockets+SocketError")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum SocketError {
    #[default]
    AccessDenied = 10013i32,
    AddressAlreadyInUse = 10048i32,
    AddressFamilyNotSupported = 10047i32,
    AddressNotAvailable = 10049i32,
    AlreadyInProgress = 10037i32,
    ConnectionAborted = 10053i32,
    ConnectionRefused = 10061i32,
    ConnectionReset = 10054i32,
    DestinationAddressRequired = 10039i32,
    Disconnecting = 10101i32,
    Fault = 10014i32,
    HostDown = 10064i32,
    HostNotFound = 11001i32,
    HostUnreachable = 10065i32,
    IOPending = 997i32,
    InProgress = 10036i32,
    Interrupted = 10004i32,
    InvalidArgument = 10022i32,
    IsConnected = 10056i32,
    MessageSize = 10040i32,
    NetworkDown = 10050i32,
    NetworkReset = 10052i32,
    NetworkUnreachable = 10051i32,
    NoBufferSpaceAvailable = 10055i32,
    NoData = 11004i32,
    NoRecovery = 11003i32,
    NotConnected = 10057i32,
    NotInitialized = 10093i32,
    NotSocket = 10038i32,
    OperationAborted = 995i32,
    OperationNotSupported = 10045i32,
    ProcessLimit = 10067i32,
    ProtocolFamilyNotSupported = 10046i32,
    ProtocolNotSupported = 10043i32,
    ProtocolOption = 10042i32,
    ProtocolType = 10041i32,
    Shutdown = 10058i32,
    SocketError = -1i32,
    SocketNotSupported = 10044i32,
    Success = 0i32,
    SystemNotReady = 10091i32,
    TimedOut = 10060i32,
    TooManyOpenSockets = 10024i32,
    TryAgain = 11002i32,
    TypeNotFound = 10109i32,
    VersionNotSupported = 10092i32,
    WouldBlock = 10035i32,
}
#[cfg(feature = "cordl_class_System+Net+Sockets+SocketError")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::Sockets::SocketError {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net.Sockets";
    const CLASS_NAME: &'static str = "SocketError";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_System+Net+Sockets+SocketError")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Net::Sockets::SocketError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Net+Sockets+SocketError")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Net::Sockets::SocketError {
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
#[cfg(feature = "cordl_class_System+Net+Sockets+SocketError")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Net::Sockets::SocketError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_System+Net+Sockets+SocketError")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Net::Sockets::SocketError {
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
