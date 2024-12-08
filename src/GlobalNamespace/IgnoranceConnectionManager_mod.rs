#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceConnectionManager_ConnectToServerParams {
    __cordl_parent: crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase,
    pub hostName: *mut crate::System::String,
    pub port: i32,
    pub serverUserId: *mut crate::System::String,
    pub serverUserName: *mut crate::System::String,
    pub validateCertificate: bool,
    pub rootCertificatePath: *mut crate::System::String,
    pub rootCertificate: *mut crate::System::String,
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams => ""
    ."IgnoranceConnectionManager/ConnectToServerParams"
);
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
impl std::ops::Deref
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams {
    type Target = crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
impl crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IgnoranceConnectionManager_ConnectionState {
    Connected = 1i32,
    Connecting = 0i32,
    Disconnected = 3i32,
    Disconnecting = 2i32,
    Disposed = 4i32,
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IgnoranceConnectionManager_ConnectionState => ""
    ."IgnoranceConnectionManager/ConnectionState"
);
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceConnectionManager_IgnoranceConnection {
    __cordl_parent: crate::System::Object,
    pub _connectionManager: *mut IgnoranceConnectionManager,
    pub peerId: u32,
    pub _userId_k__BackingField: *mut crate::System::String,
    pub _userName_k__BackingField: *mut crate::System::String,
    pub _isConnectionOwner_k__BackingField: bool,
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection => ""
    ."IgnoranceConnectionManager/IgnoranceConnection"
);
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
impl std::ops::Deref
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
impl crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    pub fn Disconnect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        connectionManager: *mut IgnoranceConnectionManager,
        peerId: u32,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (connectionManager, peerId, userId, userName, isConnectionOwner),
            )?;
        Ok(__cordl_object)
    }
    pub fn Send(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (writer, deliveryMethod))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        connectionManager: *mut IgnoranceConnectionManager,
        peerId: u32,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (connectionManager, peerId, userId, userName, isConnectionOwner),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_isConnectionOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnectionOwner", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_userId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_userName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_isConnectionOwner(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isConnectionOwner", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_userId(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userId", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_userName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userName", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceConnectionManager {
    __cordl_parent: crate::System::Object,
    pub onInitializedEvent: *mut crate::System::Action,
    pub onConnectedEvent: *mut crate::System::Action,
    pub onDisconnectedEvent: *mut crate::System::Action_1<DisconnectedReason>,
    pub onConnectionFailedEvent: *mut crate::System::Action_1<ConnectionFailedReason>,
    pub onConnectionConnectedEvent: *mut crate::System::Action_1<*mut IConnection>,
    pub onConnectionDisconnectedEvent: *mut crate::System::Action_2<
        *mut IConnection,
        DisconnectedReason,
    >,
    pub onReceivedDataEvent: *mut crate::System::Action_3<
        *mut IConnection,
        *mut crate::LiteNetLib::Utils::NetDataReader,
        crate::BGNet::Core::DeliveryMethod,
    >,
    pub _userId_k__BackingField: *mut crate::System::String,
    pub _userName_k__BackingField: *mut crate::System::String,
    pub _channels: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::IgnoranceCore::IgnoranceChannelTypes,
    >,
    pub _state: crate::GlobalNamespace::IgnoranceConnectionManager_ConnectionState,
    pub _client: *mut crate::IgnoranceCore::IgnoranceClient,
    pub _server: *mut crate::IgnoranceCore::IgnoranceServer,
    pub _connectionRequestHandler: *mut IConnectionRequestHandler,
    pub _pendingConnections: *mut crate::System::Collections::Generic::List_1<u32>,
    pub _connections: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection,
    >,
    pub _incomingDataWriter: *mut crate::LiteNetLib::Utils::NetDataWriter,
    pub _incomingDataReader: *mut crate::LiteNetLib::Utils::NetDataReader,
    pub _connectionRequestWriter: *mut crate::LiteNetLib::Utils::NetDataWriter,
    pub _backgroundSentryDisconnectCts: *mut crate::System::Threading::CancellationTokenSource,
    pub _sentryDisconnected: bool,
    pub _lastPollUpdateTime: i64,
    pub _timeProvider: *mut crate::BGNet::Core::ITimeProvider,
    pub _taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    pub _serverUserId_k__BackingField: *mut crate::System::String,
    pub _serverUserName_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "IgnoranceConnectionManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IgnoranceConnectionManager => ""
    ."IgnoranceConnectionManager"
);
#[cfg(feature = "IgnoranceConnectionManager")]
impl std::ops::Deref for IgnoranceConnectionManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl std::ops::DerefMut for IgnoranceConnectionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl IgnoranceConnectionManager {
    pub const kBackgroundDisconnectTimeoutMs: i64 = 120000i64;
    pub const kIgnoranceConnectHeader: &'static str = "IgnCon";
    pub const kMaxClientShutdownTimeMs: i32 = 30i32;
    pub const kMaxServerShutdownTimeMs: i32 = 300i32;
    #[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
    pub type IgnoranceConnectionParamsBase = crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase;
    #[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
    pub type ConnectToServerParams = crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams;
    #[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
    pub type StartServerParams = crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams;
    #[cfg(feature = "IgnoranceConnectionManager+__c__DisplayClass77_0")]
    pub type __c__DisplayClass77_0 = crate::GlobalNamespace::IgnoranceConnectionManager___c__DisplayClass77_0;
    #[cfg(feature = "IgnoranceConnectionManager+ConnectionState")]
    pub type ConnectionState = crate::GlobalNamespace::IgnoranceConnectionManager_ConnectionState;
    #[cfg(feature = "IgnoranceConnectionManager+_BackgroundDisconnectSentry_d__99")]
    pub type _BackgroundDisconnectSentry_d__99 = crate::GlobalNamespace::IgnoranceConnectionManager__BackgroundDisconnectSentry_d__99;
    #[cfg(feature = "IgnoranceConnectionManager+__c__DisplayClass77_1")]
    pub type __c__DisplayClass77_1 = crate::GlobalNamespace::IgnoranceConnectionManager___c__DisplayClass77_1;
    #[cfg(feature = "IgnoranceConnectionManager+_DisposeAsync_d__77")]
    pub type _DisposeAsync_d__77 = crate::GlobalNamespace::IgnoranceConnectionManager__DisposeAsync_d__77;
    #[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
    pub type IgnoranceConnection = crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection;
    pub fn BackgroundDisconnectSentry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("BackgroundDisconnectSentry", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckSentryState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckSentryState", ())?;
        Ok(__cordl_ret)
    }
    pub fn Disconnect(
        &mut self,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", (disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectPeer(
        &mut self,
        peerId: u32,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectPeer", (peerId, disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("DisposeAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetConnection(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut IConnection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IConnection = __cordl_object
            .invoke("GetConnection", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetConnectionMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::Utils::NetDataWriter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::Utils::NetDataWriter = __cordl_object
            .invoke("GetConnectionMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleConnectionEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleDisconnectionEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDisconnectionEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleIncomingEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleIncomingEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init<T>(
        &mut self,
        initParams: *mut IConnectionInitParams_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Init", (initParams))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_ITimeProvider_ITaskUtility1(
        timeProvider: *mut crate::BGNet::Core::ITimeProvider,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (timeProvider, taskUtility))?;
        Ok(__cordl_object)
    }
    pub fn PollUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Send(
        &mut self,
        peerId: u32,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (peerId, writer, deliveryMethod))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_IConnection1(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
        excludingConnection: *mut IConnection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (writer, deliveryMethod, excludingConnection))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_NetDataWriter_DeliveryMethod0(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (writer, deliveryMethod))?;
        Ok(__cordl_ret)
    }
    pub fn StartBackgroundSentry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartBackgroundSentry", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryGetConnection(
        &mut self,
        peerId: u32,
        connection: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetConnection", (peerId, connection))?;
        Ok(__cordl_ret)
    }
    pub fn TryParseConnectionMessage(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        userId: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        userName: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        isConnectionOwner: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryParseConnectionMessage",
                (reader, userId, userName, isConnectionOwner),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ITimeProvider_ITaskUtility1(
        &mut self,
        timeProvider: *mut crate::BGNet::Core::ITimeProvider,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (timeProvider, taskUtility))?;
        Ok(__cordl_ret)
    }
    pub fn add_onConnectedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onConnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onConnectionConnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onConnectionConnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onConnectionDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut IConnection, DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onConnectionDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onConnectionFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<ConnectionFailedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onConnectionFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onInitializedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onInitializedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onReceivedDataEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut IConnection,
            *mut crate::LiteNetLib::Utils::NetDataReader,
            crate::BGNet::Core::DeliveryMethod,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onReceivedDataEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_connectionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_connectionCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isConnecting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnecting", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isConnectionOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnectionOwner", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isDisconnecting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDisconnecting", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isDisposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDisposed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_serverUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_serverUserId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_serverUserName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_serverUserName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_userId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_userName", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_onConnectedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onConnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onConnectionConnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onConnectionConnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onConnectionDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut IConnection, DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onConnectionDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onConnectionFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<ConnectionFailedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onConnectionFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onInitializedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onInitializedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onReceivedDataEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut IConnection,
            *mut crate::LiteNetLib::Utils::NetDataReader,
            crate::BGNet::Core::DeliveryMethod,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onReceivedDataEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_serverUserId(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_serverUserId", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_serverUserName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_serverUserName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_userId(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userId", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_userName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userName", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl quest_hook::libil2cpp::ObjectType for IgnoranceConnectionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    __cordl_parent: crate::System::Object,
    pub connectionRequestHandler: *mut IConnectionRequestHandler,
    pub useSsl: bool,
    pub userId: *mut crate::System::String,
    pub userName: *mut crate::System::String,
    pub enableBackgroundSentry: bool,
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase => ""
    ."IgnoranceConnectionManager/IgnoranceConnectionParamsBase"
);
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
impl std::ops::Deref
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
impl crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceConnectionManager_StartServerParams {
    __cordl_parent: crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase,
    pub port: i32,
    pub certificatePath: *mut crate::System::String,
    pub certificate: *mut crate::System::String,
    pub privateKeyPath: *mut crate::System::String,
    pub privateKey: *mut crate::System::String,
}
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IgnoranceConnectionManager_StartServerParams => ""
    ."IgnoranceConnectionManager/StartServerParams"
);
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
impl std::ops::Deref
for crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams {
    type Target = crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
impl crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
