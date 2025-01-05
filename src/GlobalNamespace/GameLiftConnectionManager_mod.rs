#[cfg(feature = "GameLiftConnectionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftConnectionManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
    pub _taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    pub _connectionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectionManager,
    >,
    pub _certificateValidator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ICertificateValidator,
    >,
    pub _code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    pub _configuration: crate::GlobalNamespace::GameplayServerConfiguration,
    pub _connectionState: crate::GlobalNamespace::GameLiftConnectionManager_ConnectionState,
    pub _connectionCancellationTokenSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
    pub _authenticationTokenProviderTask: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IAuthenticationTokenProvider>,
    >,
    pub _gameLiftPlayerSessionProvider: quest_hook::libil2cpp::Gc<
        crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider,
    >,
    pub _connectionRequestHandler: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameLiftClientConnectionRequestHandler,
    >,
    pub onInitializedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub onConnectedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub onDisconnectedEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DisconnectedReason,
    >,
    pub onConnectionFailedEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ConnectionFailedReason,
    >,
    pub onConnectionConnectedEvent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
    >,
    pub onConnectionDisconnectedEvent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        crate::GlobalNamespace::DisconnectedReason,
    >,
    pub onReceivedDataEvent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        crate::BGNet::Core::DeliveryMethod,
    >,
}
#[cfg(feature = "GameLiftConnectionManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameLiftConnectionManager => ""
    ."GameLiftConnectionManager"
);
#[cfg(feature = "GameLiftConnectionManager")]
impl std::ops::Deref for crate::GlobalNamespace::GameLiftConnectionManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameLiftConnectionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl crate::GlobalNamespace::GameLiftConnectionManager {
    #[cfg(feature = "GameLiftConnectionManager+ConnectToServerParams")]
    pub type ConnectToServerParams = crate::GlobalNamespace::GameLiftConnectionManager_ConnectToServerParams;
    #[cfg(feature = "GameLiftConnectionManager+ConnectionState")]
    pub type ConnectionState = crate::GlobalNamespace::GameLiftConnectionManager_ConnectionState;
    #[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
    pub type GameLiftConnectionManagerParamsBase = crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase;
    #[cfg(feature = "GameLiftConnectionManager+StartClientParams")]
    pub type StartClientParams = crate::GlobalNamespace::GameLiftConnectionManager_StartClientParams;
    pub fn Disconnect(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", (disconnectedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisconnectInternal_ConnectionFailedReason0(
        &mut self,
        connectionFailedReason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectInternal", (connectionFailedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisconnectInternal_DisconnectedReason_ConnectionFailedReason1(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
        connectionFailedReason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectInternal", (disconnectedReason, connectionFailedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("DisposeAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GameLiftConnectToServer(
        &mut self,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GameLiftConnectToServer", (secret, code, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConnection(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnection,
        > = __cordl_object.invoke("GetConnection", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPublicServers(
        &mut self,
        onSuccess: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PublicServerInfo>,
        >,
        onFailure: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectionFailedReason,
        >,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        configuration: crate::GlobalNamespace::GameplayServerConfiguration,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleConnectToServerSuccess(
        &mut self,
        playerSessionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        port: i32,
        gameSessionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        configuration: crate::GlobalNamespace::GameplayServerConfiguration,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleConnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleConnectionConnected(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionConnected", (connection))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleConnectionDisconnected(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        reason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionDisconnected", (connection, reason))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleConnectionFailed(
        &mut self,
        failedReason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionFailed", (failedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleReceivedData(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleReceivedData", (connection, reader, deliveryMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init<T>(
        &mut self,
        initParams: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Init", (initParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc_Gc_Gc_Gc1(
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
        certificateValidator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICertificateValidator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (timeProvider, taskUtility, connectionManager, certificateValidator),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn PollUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendToAll_Gc1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
        excludingConnection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (writer, deliveryMethod, excludingConnection))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendToAll_Gc_DeliveryMethod0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (writer, deliveryMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc_Gc_Gc_Gc1(
        &mut self,
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
        certificateValidator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICertificateValidator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (timeProvider, taskUtility, connectionManager, certificateValidator),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onConnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onConnectionConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onConnectionConnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onConnectionDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
            crate::GlobalNamespace::DisconnectedReason,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onConnectionDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onConnectionFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ConnectionFailedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onConnectionFailedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onInitializedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onInitializedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onReceivedDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
            crate::BGNet::Core::DeliveryMethod,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onReceivedDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_code(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_code", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_configuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayServerConfiguration,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::GameplayServerConfiguration = __cordl_object
            .invoke("get_configuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_connectionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_connectionCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnecting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnecting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnectionOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnectionOwner", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDisconnecting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDisconnecting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDisposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDisposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerSessionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_playerSessionId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_secret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_secret", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectionMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapLevelSelectionMask,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelSelectionMask = __cordl_object
            .invoke("get_selectionMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_userId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_userName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onConnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onConnectionConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onConnectionConnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onConnectionDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
            crate::GlobalNamespace::DisconnectedReason,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onConnectionDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onConnectionFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ConnectionFailedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onConnectionFailedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onInitializedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onInitializedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onReceivedDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
            crate::BGNet::Core::DeliveryMethod,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onReceivedDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameLiftConnectionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectionManager>>
for crate::GlobalNamespace::GameLiftConnectionManager {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectionManager> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectionManager>>
for crate::GlobalNamespace::GameLiftConnectionManager {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectionManager> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable>>
for crate::GlobalNamespace::GameLiftConnectionManager {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable>>
for crate::GlobalNamespace::GameLiftConnectionManager {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::GameLiftConnectionManager {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameLiftConnectionManager")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::GameLiftConnectionManager {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameLiftConnectionManager+ConnectToServerParams")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftConnectionManager_ConnectToServerParams {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase,
    >,
    pub secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase,
    >;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameLiftConnectionManager_ConnectionState {
    #[default]
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
#[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftConnectionManager_GameLiftConnectionManagerParamsBase {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub authenticationTokenProviderTask: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IAuthenticationTokenProvider>,
    >,
    pub gameLiftPlayerSessionProvider: quest_hook::libil2cpp::Gc<
        crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider,
    >,
    pub selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    pub configuration: crate::GlobalNamespace::GameplayServerConfiguration,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameLiftConnectionManager>,
    >,
>
for crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameLiftConnectionManager>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameLiftConnectionManager+GameLiftConnectionManagerParamsBase")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameLiftConnectionManager>,
    >,
>
for crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameLiftConnectionManager>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameLiftConnectionManager+StartClientParams")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftConnectionManager_StartClientParams {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase,
    >,
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
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameLiftConnectionManager_GameLiftConnectionManagerParamsBase,
    >;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
