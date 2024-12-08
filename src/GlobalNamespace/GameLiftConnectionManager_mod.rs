#[cfg(feature = "GameLiftConnectionManager+ConnectToServerParams")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftConnectionManager_ConnectToServerParams {
    __cordl_parent: crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase,
    pub secret: *mut crate::System::String,
    pub code: *mut crate::System::String,
}
#[cfg(feature = "GameLiftConnectionManager+ConnectToServerParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameLiftConnectionManager_ConnectToServerParams => ""
    ."GameLiftConnectionManager/ConnectToServerParams"
);
#[cfg(feature = "GameLiftConnectionManager+ConnectToServerParams")]
impl std::ops::Deref
for crate::GlobalNamespace::GameLiftConnectionManager_ConnectToServerParams {
    type Target = crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftConnectionManager+ConnectToServerParams")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameLiftConnectionManager_ConnectToServerParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftConnectionManager+ConnectToServerParams")]
impl crate::GlobalNamespace::GameLiftConnectionManager_ConnectToServerParams {
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
#[cfg(feature = "GameLiftConnectionManager+ConnectToServerParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameLiftConnectionManager_ConnectToServerParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameLiftConnectionManager+ConnectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameLiftConnectionManager_ConnectionState {
    Connected = 2i32,
    Connecting = 1i32,
    Disconnecting = 3i32,
    Unconnected = 0i32,
}
#[cfg(feature = "GameLiftConnectionManager+ConnectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameLiftConnectionManager_ConnectionState => ""
    ."GameLiftConnectionManager/ConnectionState"
);
#[cfg(feature = "GameLiftConnectionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftConnectionManager {
    __cordl_parent: crate::System::Object,
    pub _timeProvider: *mut crate::BGNet::Core::ITimeProvider,
    pub _taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    pub _connectionManager: *mut IConnectionManager,
    pub _certificateValidator: *mut ICertificateValidator,
    pub _code: *mut crate::System::String,
    pub _secret: *mut crate::System::String,
    pub _selectionMask: BeatmapLevelSelectionMask,
    pub _configuration: GameplayServerConfiguration,
    pub _connectionState: crate::GlobalNamespace::GameLiftConnectionManager_ConnectionState,
    pub _connectionCancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _authenticationTokenProviderTask: *mut crate::System::Threading::Tasks::Task_1<
        *mut IAuthenticationTokenProvider,
    >,
    pub _gameLiftPlayerSessionProvider: *mut crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider,
    pub _connectionRequestHandler: *mut GameLiftClientConnectionRequestHandler,
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
}
#[cfg(feature = "GameLiftConnectionManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameLiftConnectionManager => ""
    ."GameLiftConnectionManager"
);
#[cfg(feature = "GameLiftConnectionManager")]
impl std::ops::Deref for GameLiftConnectionManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl std::ops::DerefMut for GameLiftConnectionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl GameLiftConnectionManager {
    #[cfg(feature = "GameLiftConnectionManager+ConnectToServerParams")]
    pub type ConnectToServerParams = crate::GlobalNamespace::GameLiftConnectionManager_ConnectToServerParams;
    #[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
    pub type GameLiftConnectionManagerParamsBase = crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase;
    #[cfg(feature = "GameLiftConnectionManager+StartClientParams")]
    pub type StartClientParams = crate::GlobalNamespace::GameLiftConnectionManager_StartClientParams;
    #[cfg(feature = "GameLiftConnectionManager+ConnectionState")]
    pub type ConnectionState = crate::GlobalNamespace::GameLiftConnectionManager_ConnectionState;
    #[cfg(feature = "GameLiftConnectionManager+_GameLiftConnectToServer_d__81")]
    pub type _GameLiftConnectToServer_d__81 = crate::GlobalNamespace::GameLiftConnectionManager__GameLiftConnectToServer_d__81;
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
    pub fn DisconnectInternal_ConnectionFailedReason0(
        &mut self,
        connectionFailedReason: ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectInternal", (connectionFailedReason))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectInternal_DisconnectedReason_ConnectionFailedReason1(
        &mut self,
        disconnectedReason: DisconnectedReason,
        connectionFailedReason: ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectInternal", (disconnectedReason, connectionFailedReason))?;
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
    pub fn GameLiftConnectToServer(
        &mut self,
        secret: *mut crate::System::String,
        code: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GameLiftConnectToServer", (secret, code, cancellationToken))?;
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
    pub fn GetPublicServers(
        &mut self,
        onSuccess: *mut crate::System::Action_1<
            *mut crate::System::Collections::Generic::IReadOnlyList_1<PublicServerInfo>,
        >,
        onFailure: *mut crate::System::Action_1<ConnectionFailedReason>,
        selectionMask: BeatmapLevelSelectionMask,
        configuration: GameplayServerConfiguration,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetPublicServers",
                (onSuccess, onFailure, selectionMask, configuration, offset, count),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleConnectToServerSuccess(
        &mut self,
        playerSessionId: *mut crate::System::String,
        hostName: *mut crate::System::String,
        port: i32,
        gameSessionId: *mut crate::System::String,
        secret: *mut crate::System::String,
        code: *mut crate::System::String,
        selectionMask: BeatmapLevelSelectionMask,
        configuration: GameplayServerConfiguration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleConnectToServerSuccess",
                (
                    playerSessionId,
                    hostName,
                    port,
                    gameSessionId,
                    secret,
                    code,
                    selectionMask,
                    configuration,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleConnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleConnectionConnected(
        &mut self,
        connection: *mut IConnection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionConnected", (connection))?;
        Ok(__cordl_ret)
    }
    pub fn HandleConnectionDisconnected(
        &mut self,
        connection: *mut IConnection,
        reason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionDisconnected", (connection, reason))?;
        Ok(__cordl_ret)
    }
    pub fn HandleConnectionFailed(
        &mut self,
        failedReason: ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionFailed", (failedReason))?;
        Ok(__cordl_ret)
    }
    pub fn HandleReceivedData(
        &mut self,
        connection: *mut IConnection,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleReceivedData", (connection, reader, deliveryMethod))?;
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
    pub fn New_ITimeProvider_ITaskUtility_IConnectionManager_ICertificateValidator1(
        timeProvider: *mut crate::BGNet::Core::ITimeProvider,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
        connectionManager: *mut IConnectionManager,
        certificateValidator: *mut ICertificateValidator,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (timeProvider, taskUtility, connectionManager, certificateValidator),
            )?;
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
    pub fn _ctor_ITimeProvider_ITaskUtility_IConnectionManager_ICertificateValidator1(
        &mut self,
        timeProvider: *mut crate::BGNet::Core::ITimeProvider,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
        connectionManager: *mut IConnectionManager,
        certificateValidator: *mut ICertificateValidator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (timeProvider, taskUtility, connectionManager, certificateValidator),
            )?;
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
    pub fn get_code(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_code", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_configuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<GameplayServerConfiguration> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: GameplayServerConfiguration = __cordl_object
            .invoke("get_configuration", ())?;
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
    pub fn get_playerSessionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_playerSessionId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_secret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_secret", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<BeatmapLevelSelectionMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapLevelSelectionMask = __cordl_object
            .invoke("get_selectionMask", ())?;
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
}
#[cfg(feature = "GameLiftConnectionManager")]
impl quest_hook::libil2cpp::ObjectType for GameLiftConnectionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftConnectionManager_GameLiftConnectionManagerParamsBase {
    __cordl_parent: crate::System::Object,
    pub authenticationTokenProviderTask: *mut crate::System::Threading::Tasks::Task_1<
        *mut IAuthenticationTokenProvider,
    >,
    pub gameLiftPlayerSessionProvider: *mut crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider,
    pub selectionMask: BeatmapLevelSelectionMask,
    pub configuration: GameplayServerConfiguration,
}
#[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase =>
    ""."GameLiftConnectionManager/GameLiftConnectionManagerParamsBase"
);
#[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
impl std::ops::Deref
for crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
impl crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase {
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
#[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameLiftConnectionManager+StartClientParams")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftConnectionManager_StartClientParams {
    __cordl_parent: crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase,
}
#[cfg(feature = "GameLiftConnectionManager+StartClientParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameLiftConnectionManager_StartClientParams => ""
    ."GameLiftConnectionManager/StartClientParams"
);
#[cfg(feature = "GameLiftConnectionManager+StartClientParams")]
impl std::ops::Deref
for crate::GlobalNamespace::GameLiftConnectionManager_StartClientParams {
    type Target = crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftConnectionManager+StartClientParams")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameLiftConnectionManager_StartClientParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftConnectionManager+StartClientParams")]
impl crate::GlobalNamespace::GameLiftConnectionManager_StartClientParams {
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
#[cfg(feature = "GameLiftConnectionManager+StartClientParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameLiftConnectionManager_StartClientParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}