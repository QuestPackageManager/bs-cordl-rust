#[cfg(feature = "LiteNetLibConnectionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct LiteNetLibConnectionManager {
    __cordl_parent: crate::System::Object,
    pub _unconnectedPacketHeader: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _netManager: *mut crate::LiteNetLib::NetManager,
    pub _encryptionLayer: *mut crate::GlobalNamespace::PacketEncryptionLayer,
    pub _taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    pub _connections: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection,
    >,
    pub _pendingConnections: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection,
    >,
    pub _pendingRequests: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnectionRequest,
    >,
    pub _pendingReconnections: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::Net::IPEndPoint,
    >,
    pub _userId: *mut crate::System::String,
    pub _userName: *mut crate::System::String,
    pub _connectionRequestHandler: *mut crate::GlobalNamespace::IConnectionRequestHandler,
    pub _mode: crate::GlobalNamespace::LiteNetLibConnectionManager_NetworkMode,
    pub _connectionState: crate::GlobalNamespace::LiteNetLibConnectionManager_ConnectionState,
    pub _backgroundSentryDisconnectCts: *mut crate::System::Threading::CancellationTokenSource,
    pub _backgroundSentryShutdownCts: *mut crate::System::Threading::CancellationTokenSource,
    pub _sentryDisconnected: bool,
    pub _sentryShutdown: bool,
    pub _lastPollUpdateTime: i64,
    pub _lastStatisticsUpdateTime: i64,
    pub onInitializedEvent: *mut crate::System::Action,
    pub onConnectedEvent: *mut crate::System::Action,
    pub onDisconnectedEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::DisconnectedReason,
    >,
    pub onConnectionFailedEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::ConnectionFailedReason,
    >,
    pub onConnectionConnectedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::IConnection,
    >,
    pub onConnectionDisconnectedEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::IConnection,
        crate::GlobalNamespace::DisconnectedReason,
    >,
    pub onReceivedDataEvent: *mut crate::System::Action_3<
        *mut crate::GlobalNamespace::IConnection,
        *mut crate::LiteNetLib::Utils::NetDataReader,
        crate::BGNet::Core::DeliveryMethod,
    >,
    pub onReceiveUnconnectedDataEvent: *mut crate::System::Action_2<
        *mut crate::System::Net::IPEndPoint,
        *mut crate::LiteNetLib::Utils::NetDataReader,
    >,
    pub onStatisticsUpdatedEvent: *mut crate::GlobalNamespace::NetworkStatisticsState_NetworkStatisticsUpdateDelegate,
}
#[cfg(feature = "LiteNetLibConnectionManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LiteNetLibConnectionManager =>
    ""."LiteNetLibConnectionManager"
);
#[cfg(feature = "LiteNetLibConnectionManager")]
impl std::ops::Deref for crate::GlobalNamespace::LiteNetLibConnectionManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::LiteNetLibConnectionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager")]
impl crate::GlobalNamespace::LiteNetLibConnectionManager {
    pub const kBackgroundDisconnectTimeout: i64 = 1200000000i64;
    pub const kBackgroundShutdownTimeout: i64 = 9000000000i64;
    pub const kStatisticsUpdateInterval: i64 = 300000000i64;
    #[cfg(feature = "LiteNetLibConnectionManager+ConnectToServerParams")]
    pub type ConnectToServerParams = crate::GlobalNamespace::LiteNetLibConnectionManager_ConnectToServerParams;
    #[cfg(feature = "LiteNetLibConnectionManager+ConnectionState")]
    pub type ConnectionState = crate::GlobalNamespace::LiteNetLibConnectionManager_ConnectionState;
    #[cfg(feature = "LiteNetLibConnectionManager+LiteNetLibConnectionParamsBase")]
    pub type LiteNetLibConnectionParamsBase = crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase;
    #[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnection")]
    pub type NetPeerConnection = crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection;
    #[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnectionRequest")]
    pub type NetPeerConnectionRequest = crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnectionRequest;
    #[cfg(feature = "LiteNetLibConnectionManager+NetworkMode")]
    pub type NetworkMode = crate::GlobalNamespace::LiteNetLibConnectionManager_NetworkMode;
    #[cfg(feature = "LiteNetLibConnectionManager+StartClientParams")]
    pub type StartClientParams = crate::GlobalNamespace::LiteNetLibConnectionManager_StartClientParams;
    #[cfg(feature = "LiteNetLibConnectionManager+StartServerParams")]
    pub type StartServerParams = crate::GlobalNamespace::LiteNetLibConnectionManager_StartServerParams;
    #[cfg(feature = "LiteNetLibConnectionManager+_BackgroundDisconnectSentry_d__126")]
    pub type _BackgroundDisconnectSentry_d__126 = crate::GlobalNamespace::LiteNetLibConnectionManager__BackgroundDisconnectSentry_d__126;
    #[cfg(feature = "LiteNetLibConnectionManager+_BackgroundShutdownSentry_d__127")]
    pub type _BackgroundShutdownSentry_d__127 = crate::GlobalNamespace::LiteNetLibConnectionManager__BackgroundShutdownSentry_d__127;
    #[cfg(feature = "LiteNetLibConnectionManager+__c")]
    pub type __c = crate::GlobalNamespace::LiteNetLibConnectionManager___c;
    pub fn AcceptAllPendingRequests(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AcceptAllPendingRequests", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn BackgroundShutdownSentry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("BackgroundShutdownSentry", ())?;
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
    pub fn ConnectToEndPoint(
        &mut self,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        remoteUserId: *mut crate::System::String,
        remoteUserName: *mut crate::System::String,
        remoteUserIsConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ConnectToEndPoint",
                (
                    userId,
                    userName,
                    remoteEndPoint,
                    remoteUserId,
                    remoteUserName,
                    remoteUserIsConnectionOwner,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreatePendingConnection(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CreatePendingConnection",
                (peer, userId, userName, isConnectionOwner),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Disconnect(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", (disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectInternal(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
        connectionFailedReason: crate::GlobalNamespace::ConnectionFailedReason,
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
    pub fn DisposeInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisposeInternal", ())?;
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
    pub fn GetConnection_NetPeer1(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection = __cordl_object
            .invoke("GetConnection", (peer))?;
        Ok(__cordl_ret)
    }
    pub fn GetConnection_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::IConnection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IConnection = __cordl_object
            .invoke("GetConnection", (index))?;
        Ok(__cordl_ret)
    }
    pub fn HasConnectionToEndPoint(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasConnectionToEndPoint", (endPoint))?;
        Ok(__cordl_ret)
    }
    pub fn HasPendingConnectionToEndPoint(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasPendingConnectionToEndPoint", (endPoint))?;
        Ok(__cordl_ret)
    }
    pub fn Init<T>(
        &mut self,
        initParams: *mut crate::GlobalNamespace::IConnectionInitParams_1<T>,
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
    pub fn IsConnectedToUser(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsConnectedToUser", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_INetEventListener_OnConnectionRequest(
        &mut self,
        request: *mut crate::LiteNetLib::ConnectionRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LiteNetLib.INetEventListener.OnConnectionRequest", (request))?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_INetEventListener_OnNetworkError(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        socketError: crate::System::Net::Sockets::SocketError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetEventListener.OnNetworkError",
                (endPoint, socketError),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_INetEventListener_OnNetworkLatencyUpdate(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        latencyMs: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetEventListener.OnNetworkLatencyUpdate",
                (peer, latencyMs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_INetEventListener_OnNetworkReceive(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        reader: *mut crate::LiteNetLib::NetPacketReader,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetEventListener.OnNetworkReceive",
                (peer, reader, deliveryMethod),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_INetEventListener_OnNetworkReceiveUnconnected(
        &mut self,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        reader: *mut crate::LiteNetLib::NetPacketReader,
        messageType: crate::LiteNetLib::UnconnectedMessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetEventListener.OnNetworkReceiveUnconnected",
                (remoteEndPoint, reader, messageType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_INetEventListener_OnPeerConnected(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LiteNetLib.INetEventListener.OnPeerConnected", (peer))?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_INetEventListener_OnPeerDisconnected(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        disconnectInfo: crate::LiteNetLib::DisconnectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetEventListener.OnPeerDisconnected",
                (peer, disconnectInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Log(
        &mut self,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn LogError(
        &mut self,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogError", (msg))?;
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
    pub fn RemoveConnection(
        &mut self,
        netPeer: *mut crate::LiteNetLib::NetPeer,
        reason: crate::LiteNetLib::DisconnectReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveConnection", (netPeer, reason))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_IConnection1(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
        excludingConnection: *mut crate::GlobalNamespace::IConnection,
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
    pub fn SendUnconnectedMessage(
        &mut self,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendUnconnectedMessage", (remoteEndPoint, writer))?;
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
    pub fn ToConnectionFailedReason(
        &mut self,
        disconnectReason: crate::LiteNetLib::DisconnectReason,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ConnectionFailedReason> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ConnectionFailedReason = __cordl_object
            .invoke("ToConnectionFailedReason", (disconnectReason))?;
        Ok(__cordl_ret)
    }
    pub fn ToDisconnectedReason(
        &mut self,
        disconnectReason: crate::LiteNetLib::DisconnectReason,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::DisconnectedReason> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::DisconnectedReason = __cordl_object
            .invoke("ToDisconnectedReason", (disconnectReason))?;
        Ok(__cordl_ret)
    }
    pub fn TryAccept(
        &mut self,
        request: *mut crate::LiteNetLib::ConnectionRequest,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryAccept", (request, userId, userName, isConnectionOwner))?;
        Ok(__cordl_ret)
    }
    pub fn TryDisconnect(
        &mut self,
        reason: crate::LiteNetLib::DisconnectReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryDisconnect", (reason))?;
        Ok(__cordl_ret)
    }
    pub fn TryStartNetManager(
        &mut self,
        port: i32,
        enableBackgroundSentry: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryStartNetManager", (port, enableBackgroundSentry))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStatistics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStatistics", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DisposeAsync_b__98_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DisposeAsync>b__98_0", ())?;
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
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::IConnection>,
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
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::IConnection,
            crate::GlobalNamespace::DisconnectedReason,
        >,
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
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::ConnectionFailedReason,
        >,
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
        value: *mut crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
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
    pub fn add_onReceiveUnconnectedDataEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::Net::IPEndPoint,
            *mut crate::LiteNetLib::Utils::NetDataReader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onReceiveUnconnectedDataEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onReceivedDataEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::GlobalNamespace::IConnection,
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
    pub fn add_onStatisticsUpdatedEvent(
        &mut self,
        value: *mut crate::GlobalNamespace::NetworkStatisticsState_NetworkStatisticsUpdateDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onStatisticsUpdatedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_connectionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_connectionCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_connectionRequestHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::IConnectionRequestHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IConnectionRequestHandler = __cordl_object
            .invoke("get_connectionRequestHandler", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_encryptionLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PacketEncryptionLayer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PacketEncryptionLayer = __cordl_object
            .invoke("get_encryptionLayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasConnectionOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasConnectionOwner", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isClient(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isClient", ())?;
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
    pub fn get_isServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isServer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_port(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_port", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_unconnectedPacketHeader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_unconnectedPacketHeader", ())?;
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
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::IConnection>,
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
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::IConnection,
            crate::GlobalNamespace::DisconnectedReason,
        >,
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
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::ConnectionFailedReason,
        >,
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
        value: *mut crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
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
    pub fn remove_onReceiveUnconnectedDataEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::Net::IPEndPoint,
            *mut crate::LiteNetLib::Utils::NetDataReader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onReceiveUnconnectedDataEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onReceivedDataEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::GlobalNamespace::IConnection,
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
    pub fn remove_onStatisticsUpdatedEvent(
        &mut self,
        value: *mut crate::GlobalNamespace::NetworkStatisticsState_NetworkStatisticsUpdateDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onStatisticsUpdatedEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLibConnectionManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LiteNetLibConnectionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+ConnectToServerParams")]
#[repr(C)]
#[derive(Debug)]
pub struct LiteNetLibConnectionManager_ConnectToServerParams {
    __cordl_parent: crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase,
    pub userId: *mut crate::System::String,
    pub userName: *mut crate::System::String,
    pub endPoint: *mut crate::System::Net::IPEndPoint,
    pub serverUserId: *mut crate::System::String,
    pub serverUserName: *mut crate::System::String,
    pub serverIsConnectionOwner: bool,
}
#[cfg(feature = "LiteNetLibConnectionManager+ConnectToServerParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LiteNetLibConnectionManager_ConnectToServerParams => ""
    ."LiteNetLibConnectionManager/ConnectToServerParams"
);
#[cfg(feature = "LiteNetLibConnectionManager+ConnectToServerParams")]
impl std::ops::Deref
for crate::GlobalNamespace::LiteNetLibConnectionManager_ConnectToServerParams {
    type Target = crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+ConnectToServerParams")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LiteNetLibConnectionManager_ConnectToServerParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+ConnectToServerParams")]
impl crate::GlobalNamespace::LiteNetLibConnectionManager_ConnectToServerParams {
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
#[cfg(feature = "LiteNetLibConnectionManager+ConnectToServerParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LiteNetLibConnectionManager_ConnectToServerParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+ConnectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteNetLibConnectionManager_ConnectionState {
    Connected = 2i32,
    Connecting = 1i32,
    Disconnecting = 3i32,
    Unconnected = 0i32,
}
#[cfg(feature = "LiteNetLibConnectionManager+ConnectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LiteNetLibConnectionManager_ConnectionState => ""
    ."LiteNetLibConnectionManager/ConnectionState"
);
#[cfg(feature = "LiteNetLibConnectionManager+LiteNetLibConnectionParamsBase")]
#[repr(C)]
#[derive(Debug)]
pub struct LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase {
    __cordl_parent: crate::System::Object,
    pub connectionRequestHandler: *mut crate::GlobalNamespace::IConnectionRequestHandler,
    pub port: i32,
    pub filterUnencryptedTraffic: bool,
    pub enableUnconnectedMessages: bool,
    pub enableBackgroundSentry: bool,
    pub enableStatistics: bool,
    pub disconnectTimeoutMs: i32,
}
#[cfg(feature = "LiteNetLibConnectionManager+LiteNetLibConnectionParamsBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase => ""
    ."LiteNetLibConnectionManager/LiteNetLibConnectionParamsBase"
);
#[cfg(feature = "LiteNetLibConnectionManager+LiteNetLibConnectionParamsBase")]
impl std::ops::Deref
for crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+LiteNetLibConnectionParamsBase")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+LiteNetLibConnectionParamsBase")]
impl crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase {
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
#[cfg(feature = "LiteNetLibConnectionManager+LiteNetLibConnectionParamsBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct LiteNetLibConnectionManager_NetPeerConnection {
    __cordl_parent: crate::System::Object,
    pub _userId: *mut crate::System::String,
    pub _userName: *mut crate::System::String,
    pub _isConnectionOwner: bool,
    pub netPeer: *mut crate::LiteNetLib::NetPeer,
}
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection => ""
    ."LiteNetLibConnectionManager/NetPeerConnection"
);
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnection")]
impl std::ops::Deref
for crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnection")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnection")]
impl crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection {
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
    pub fn Equals_LiteNetLibConnectionManager_NetPeerConnection0(
        &mut self,
        other: *mut crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        netPeer: *mut crate::LiteNetLib::NetPeer,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (netPeer, userId, userName, isConnectionOwner))?;
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
        netPeer: *mut crate::LiteNetLib::NetPeer,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (netPeer, userId, userName, isConnectionOwner))?;
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
}
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnection")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnectionRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct LiteNetLibConnectionManager_NetPeerConnectionRequest {
    __cordl_parent: crate::System::Object,
    pub _userId: *mut crate::System::String,
    pub _userName: *mut crate::System::String,
    pub _isConnectionOwner: bool,
    pub _request: *mut crate::LiteNetLib::ConnectionRequest,
}
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnectionRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnectionRequest => ""
    ."LiteNetLibConnectionManager/NetPeerConnectionRequest"
);
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnectionRequest")]
impl std::ops::Deref
for crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnectionRequest {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnectionRequest")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnectionRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnectionRequest")]
impl crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnectionRequest {
    pub fn Accept(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::NetPeer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::NetPeer = __cordl_object
            .invoke("Accept", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        request: *mut crate::LiteNetLib::ConnectionRequest,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (request, userId, userName, isConnectionOwner))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        request: *mut crate::LiteNetLib::ConnectionRequest,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (request, userId, userName, isConnectionOwner))?;
        Ok(__cordl_ret)
    }
    pub fn get_endPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IPEndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IPEndPoint = __cordl_object
            .invoke("get_endPoint", ())?;
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
}
#[cfg(feature = "LiteNetLibConnectionManager+NetPeerConnectionRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LiteNetLibConnectionManager_NetPeerConnectionRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+NetworkMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteNetLibConnectionManager_NetworkMode {
    Client = 1i32,
    None = 0i32,
    Server = 2i32,
}
#[cfg(feature = "LiteNetLibConnectionManager+NetworkMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LiteNetLibConnectionManager_NetworkMode => ""
    ."LiteNetLibConnectionManager/NetworkMode"
);
#[cfg(feature = "LiteNetLibConnectionManager+StartClientParams")]
#[repr(C)]
#[derive(Debug)]
pub struct LiteNetLibConnectionManager_StartClientParams {
    __cordl_parent: crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase,
}
#[cfg(feature = "LiteNetLibConnectionManager+StartClientParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LiteNetLibConnectionManager_StartClientParams => ""
    ."LiteNetLibConnectionManager/StartClientParams"
);
#[cfg(feature = "LiteNetLibConnectionManager+StartClientParams")]
impl std::ops::Deref
for crate::GlobalNamespace::LiteNetLibConnectionManager_StartClientParams {
    type Target = crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+StartClientParams")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LiteNetLibConnectionManager_StartClientParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+StartClientParams")]
impl crate::GlobalNamespace::LiteNetLibConnectionManager_StartClientParams {
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
#[cfg(feature = "LiteNetLibConnectionManager+StartClientParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LiteNetLibConnectionManager_StartClientParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+StartServerParams")]
#[repr(C)]
#[derive(Debug)]
pub struct LiteNetLibConnectionManager_StartServerParams {
    __cordl_parent: crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase,
    pub userId: *mut crate::System::String,
    pub userName: *mut crate::System::String,
}
#[cfg(feature = "LiteNetLibConnectionManager+StartServerParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LiteNetLibConnectionManager_StartServerParams => ""
    ."LiteNetLibConnectionManager/StartServerParams"
);
#[cfg(feature = "LiteNetLibConnectionManager+StartServerParams")]
impl std::ops::Deref
for crate::GlobalNamespace::LiteNetLibConnectionManager_StartServerParams {
    type Target = crate::GlobalNamespace::LiteNetLibConnectionManager_LiteNetLibConnectionParamsBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+StartServerParams")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LiteNetLibConnectionManager_StartServerParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLibConnectionManager+StartServerParams")]
impl crate::GlobalNamespace::LiteNetLibConnectionManager_StartServerParams {
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
#[cfg(feature = "LiteNetLibConnectionManager+StartServerParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LiteNetLibConnectionManager_StartServerParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
