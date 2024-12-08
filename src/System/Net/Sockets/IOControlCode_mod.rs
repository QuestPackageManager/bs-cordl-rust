#[cfg(feature = "System+Net+Sockets+IOControlCode")]
#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOControlCode {
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::IOControlCode =>
    "System.Net.Sockets"."IOControlCode"
);
