#[cfg(feature = "System+Net+Sockets+IOControlCode")]
#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IOControlCode {
    #[default]
    AbsorbRouterAlert = 2550136837i64,
    AddMulticastGroupOnInterface = 2550136842i64,
    AddressListChange = 671088663i64,
    AddressListQuery = 1207959574i64,
    AddressListSort = 3355443225i64,
    AssociateHandle = 2281701377i64,
    AsyncIO = 2147772029i64,
    BindToInterface = 2550136840i64,
    DataToRead = 1074030207i64,
    DeleteMulticastGroupFromInterface = 2550136843i64,
    EnableCircularQueuing = 671088642i64,
    Flush = 671088644i64,
    GetBroadcastAddress = 1207959557i64,
    GetExtensionFunctionPointer = 3355443206i64,
    GetGroupQos = 3355443208i64,
    GetQos = 3355443207i64,
    KeepAliveValues = 2550136836i64,
    LimitBroadcasts = 2550136839i64,
    MulticastInterface = 2550136841i64,
    MulticastScope = 2281701386i64,
    MultipointLoopback = 2281701385i64,
    NamespaceChange = 2281701401i64,
    NonBlockingIO = 2147772030i64,
    OobDataRead = 1074033415i64,
    QueryTargetPnpHandle = 1207959576i64,
    ReceiveAll = 2550136833i64,
    ReceiveAllIgmpMulticast = 2550136835i64,
    ReceiveAllMulticast = 2550136834i64,
    RoutingInterfaceChange = 2281701397i64,
    RoutingInterfaceQuery = 3355443220i64,
    SetGroupQos = 2281701388i64,
    SetQos = 2281701387i64,
    TranslateHandle = 3355443213i64,
    UnicastInterface = 2550136838i64,
}
#[cfg(feature = "System+Net+Sockets+IOControlCode")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::Sockets::IOControlCode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net.Sockets";
    const CLASS_NAME: &'static str = "IOControlCode";
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
#[cfg(feature = "System+Net+Sockets+IOControlCode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::Sockets::IOControlCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+Sockets+IOControlCode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::Sockets::IOControlCode {
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
#[cfg(feature = "System+Net+Sockets+IOControlCode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::Sockets::IOControlCode {
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
#[cfg(feature = "System+Net+Sockets+IOControlCode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::Sockets::IOControlCode {
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
