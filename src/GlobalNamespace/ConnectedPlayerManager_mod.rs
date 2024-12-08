#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_ConnectedPlayer {
    __cordl_parent: crate::System::Object,
    pub _userId: *mut crate::System::String,
    pub _userName: *mut crate::System::String,
    pub _isMe: bool,
    pub _isConnectionOwner: bool,
    pub _manager: *mut ConnectedPlayerManager,
    pub _connection: *mut IConnection,
    pub _parent: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    pub _connectionId: u8,
    pub _remoteConnectionId: u8,
    pub _sortIndex: i32,
    pub _isConnected: bool,
    pub _disconnectedReason: DisconnectedReason,
    pub _isKicked: bool,
    pub _playerState: PlayerStateHash,
    pub _playerAvatars: MultiplayerAvatarsData,
    pub _publicEncryptionKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _random: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _encryptionState: *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
    pub _latency: *mut LongRollingAverage,
}
#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer => ""
    ."ConnectedPlayerManager/ConnectedPlayer"
);
#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
impl crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer {
    pub const kFixedSyncTimeOffset: i64 = 33i64;
    pub fn get_offsetSyncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_offsetSyncTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayerAvatarPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket = __cordl_object
            .invoke("GetPlayerAvatarPacket", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateState(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateState", (packet))?;
        Ok(__cordl_ret)
    }
    pub fn get_isMe(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isMe", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sortIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sortIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        manager: *mut ConnectedPlayerManager,
        connectionId: u8,
        remoteConnectionId: u8,
        connection: *mut IConnection,
        parent: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
        isMe: bool,
        publicEncryptionKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        random: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    manager,
                    connectionId,
                    remoteConnectionId,
                    connection,
                    parent,
                    userId,
                    userName,
                    isConnectionOwner,
                    isMe,
                    publicEncryptionKey,
                    random,
                ),
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
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_connectionId(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_connectionId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isKicked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isKicked", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateLatency(
        &mut self,
        latency: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLatency", (latency))?;
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
    pub fn UpdateAvatar(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatar", (packet))?;
        Ok(__cordl_ret)
    }
    pub fn get_hasValidLatency(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasValidLatency", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetKicked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKicked", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasState(
        &mut self,
        state: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasState", (state))?;
        Ok(__cordl_ret)
    }
    pub fn SetPlayerState(
        &mut self,
        playerState: PlayerStateHash,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerState", (playerState))?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayerSortOrderPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket = __cordl_object
            .invoke("GetPlayerSortOrderPacket", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isDirectConnection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDirectConnection", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayerIdentityPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket = __cordl_object
            .invoke("GetPlayerIdentityPacket", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSortIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UpdateSortIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn SetEncryptionState(
        &mut self,
        encryptionState: *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEncryptionState", (encryptionState))?;
        Ok(__cordl_ret)
    }
    pub fn get_connection(&mut self) -> quest_hook::libil2cpp::Result<*mut IConnection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IConnection = __cordl_object.invoke("get_connection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_remoteConnectionId(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_remoteConnectionId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplayerAvatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<MultiplayerAvatarsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: MultiplayerAvatarsData = __cordl_object
            .invoke("get_multiplayerAvatarsData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayerConnectedPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket = __cordl_object
            .invoke("GetPlayerConnectedPacket", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_random(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_random", ())?;
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
    pub fn get_currentLatency(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_currentLatency", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_multiplayerAvatarsData(
        &mut self,
        value: MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multiplayerAvatarsData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_encryptionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState = __cordl_object
            .invoke("get_encryptionState", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayerStatePacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket = __cordl_object
            .invoke("GetPlayerStatePacket", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateIdentity(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIdentity", (packet))?;
        Ok(__cordl_ret)
    }
    pub fn get_disconnectedReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<DisconnectedReason> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: DisconnectedReason = __cordl_object
            .invoke("get_disconnectedReason", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
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
    pub fn get_publicEncryptionKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_publicEncryptionKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        manager: *mut ConnectedPlayerManager,
        connectionId: u8,
        remoteConnectionId: u8,
        connection: *mut IConnection,
        parent: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
        isMe: bool,
        publicEncryptionKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        random: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    manager,
                    connectionId,
                    remoteConnectionId,
                    connection,
                    parent,
                    userId,
                    userName,
                    isConnectionOwner,
                    isMe,
                    publicEncryptionKey,
                    random,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager {
    __cordl_parent: crate::System::Object,
    pub connectedEvent: *mut crate::System::Action,
    pub initializedEvent: *mut crate::System::Action,
    pub disconnectedEvent: *mut crate::System::Action_1<DisconnectedReason>,
    pub connectionFailedEvent: *mut crate::System::Action_1<ConnectionFailedReason>,
    pub playerConnectedEvent: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    pub playerDisconnectedEvent: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    pub playerStateChangedEvent: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    pub playerAvatarChangedEvent: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    pub playerOrderChangedEvent: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    pub playerLatencyInitializedEvent: *mut crate::System::Action_1<
        *mut IConnectedPlayer,
    >,
    pub syncTimeInitializedEvent: *mut crate::System::Action,
    pub _startTicks: i64,
    pub _syncTimeOffset: *mut LongRollingAverage,
    pub _timeProvider: *mut crate::BGNet::Core::ITimeProvider,
    pub _taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    pub _connectionManager: *mut IConnectionManager,
    pub _temporaryDataWriter: *mut crate::LiteNetLib::Utils::NetDataWriter,
    pub _temporaryEncryptedDataWriter: *mut crate::LiteNetLib::Utils::NetDataWriter,
    pub _reliableDataWriter: *mut crate::LiteNetLib::Utils::NetDataWriter,
    pub _unreliableDataWriter: *mut crate::LiteNetLib::Utils::NetDataWriter,
    pub _players: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    >,
    pub _localPlayerState: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _localPlayerAvatars: MultiplayerAvatarsData,
    pub _encryptionKeys: *mut IDiffieHellmanKeyPair,
    pub _encryptionRandom: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _localPlayer: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    pub _lastConnectionId: u8,
    pub _lastPollTime: i64,
    pub _lastPollFrame: i32,
    pub _lastPingTime: i64,
    pub _messageSerializer: *mut NetworkPacketSerializer_2<
        crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType,
        *mut IConnectedPlayer,
    >,
}
#[cfg(feature = "ConnectedPlayerManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ConnectedPlayerManager => ""."ConnectedPlayerManager"
);
#[cfg(feature = "ConnectedPlayerManager")]
impl std::ops::Deref for ConnectedPlayerManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl std::ops::DerefMut for ConnectedPlayerManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl ConnectedPlayerManager {
    pub const invalidSortIndex: i32 = -1i32;
    pub const kAllConnectionsId: u8 = 127u8;
    pub const kLocalConnectionId: u8 = 0u8;
    pub const kMaxUnreliableMessageLength: i32 = 412i32;
    pub const kPingUpdateFrequencyMs: i64 = 2000i64;
    pub const kTimeSensitiveAllowedReceiveWindowMs: i64 = 30i64;
    #[cfg(feature = "ConnectedPlayerManager+PingPacket")]
    pub type PingPacket = crate::GlobalNamespace::ConnectedPlayerManager_PingPacket;
    #[cfg(feature = "ConnectedPlayerManager+PongPacket")]
    pub type PongPacket = crate::GlobalNamespace::ConnectedPlayerManager_PongPacket;
    #[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
    pub type PlayerIdentityPacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket;
    #[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
    pub type PlayerSortOrderPacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket;
    #[cfg(feature = "ConnectedPlayerManager+_InitializePlayerEncryption_d__132")]
    pub type _InitializePlayerEncryption_d__132 = crate::GlobalNamespace::ConnectedPlayerManager__InitializePlayerEncryption_d__132;
    #[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
    pub type PlayerDisconnectedPacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket;
    #[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
    pub type KickPlayerPacket = crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket;
    #[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
    pub type PlayerAvatarPacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket;
    #[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
    pub type SyncTimePacket = crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket;
    #[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
    pub type ConnectedPlayer = crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer;
    #[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
    pub type PlayerConnectedPacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket;
    #[cfg(feature = "ConnectedPlayerManager+MessageType")]
    pub type MessageType = crate::GlobalNamespace::ConnectedPlayerManager_MessageType;
    #[cfg(feature = "ConnectedPlayerManager+InternalMessageType")]
    pub type InternalMessageType = crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType;
    #[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
    pub type PlayerStatePacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket;
    pub fn HandleConnectionDisconnected(
        &mut self,
        connection: *mut IConnection,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionDisconnected", (connection, disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerAvatarChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerAvatarChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isConnectionOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnectionOwner", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendImmediatelyFromPlayerToPlayerUnreliable<T>(
        &mut self,
        message: T,
        fromPlayer: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        toPlayer: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendImmediatelyFromPlayerToPlayerUnreliable",
                (message, fromPlayer, toPlayer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_connectionFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<ConnectionFailedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectionFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerOrderChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerOrderChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_syncTimeInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_syncTimeInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetPlayerSortIndex(
        &mut self,
        player: *mut IConnectedPlayer,
        sortIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerSortIndex", (player, sortIndex))?;
        Ok(__cordl_ret)
    }
    pub fn SendUnreliable<T>(
        &mut self,
        message: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendUnreliable", (message))?;
        Ok(__cordl_ret)
    }
    pub fn SendUnreliableOnlyToFirstDegreeConnections<T>(
        &mut self,
        message: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendUnreliableOnlyToFirstDegreeConnections", (message))?;
        Ok(__cordl_ret)
    }
    pub fn SendImmediatelyExcludingPlayer(
        &mut self,
        message: *mut crate::LiteNetLib::Utils::INetSerializable,
        excludingPlayer: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        onlyFirstDegree: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendImmediatelyExcludingPlayer",
                (message, excludingPlayer, onlyFirstDegree),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RegisterSerializer(
        &mut self,
        serializerType: crate::GlobalNamespace::ConnectedPlayerManager_MessageType,
        subSerializer: *mut INetworkPacketSubSerializer_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterSerializer", (serializerType, subSerializer))?;
        Ok(__cordl_ret)
    }
    pub fn KickPlayer(
        &mut self,
        userId: *mut crate::System::String,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KickPlayer", (userId, disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IConnectionManager0(
        &mut self,
        connectionManager: *mut IConnectionManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectionManager))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ITimeProvider_ITaskUtility_IConnectionManager1(
        &mut self,
        timeProvider: *mut crate::BGNet::Core::ITimeProvider,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
        connectionManager: *mut IConnectionManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (timeProvider, taskUtility, connectionManager))?;
        Ok(__cordl_ret)
    }
    pub fn PollLateUpdateOptional(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollLateUpdateOptional", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLocalPlayerState(
        &mut self,
        state: *mut crate::System::String,
        setState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerState", (state, setState))?;
        Ok(__cordl_ret)
    }
    pub fn remove_syncTimeInitializedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_syncTimeInitializedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_runTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_runTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_playerDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_initializedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_initializedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNetworkReceive(
        &mut self,
        connection: *mut IConnection,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNetworkReceive", (connection, reader, deliveryMethod))?;
        Ok(__cordl_ret)
    }
    pub fn WriteOneEncrypted(
        &mut self,
        encryptionState: *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        senderId: u8,
        receiverId: u8,
        message: *mut crate::LiteNetLib::Utils::INetSerializable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::Utils::NetDataWriter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::Utils::NetDataWriter = __cordl_object
            .invoke(
                "WriteOneEncrypted",
                (encryptionState, senderId, receiverId, message),
            )?;
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
    pub fn remove_playerOrderChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerOrderChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn FlushReliableQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushReliableQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerSortOrderUpdate(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket,
        iPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerSortOrderUpdate", (packet, iPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn SendUnreliableFromPlayerToPlayer<T>(
        &mut self,
        message: T,
        fromPlayer: *mut IConnectedPlayer,
        toPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendUnreliableFromPlayerToPlayer",
                (message, fromPlayer, toPlayer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_syncTimeInitializedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_syncTimeInitializedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSyncTimePacket(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSyncTimePacket", (packet, player))?;
        Ok(__cordl_ret)
    }
    pub fn GetConnectedPlayer(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut IConnectedPlayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IConnectedPlayer = __cordl_object
            .invoke("GetConnectedPlayer", (index))?;
        Ok(__cordl_ret)
    }
    pub fn add_initializedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_initializedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_connectedPlayerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_connectedPlayerCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerAvatarUpdate(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
        iPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerAvatarUpdate", (packet, iPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn HandleKickPlayerPacket(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket,
        iPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleKickPlayerPacket", (packet, iPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn get_isDisconnecting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDisconnecting", ())?;
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
    pub fn HandleConnectionFailed(
        &mut self,
        reason: ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionFailed", (reason))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerConnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerConnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteOne(
        &mut self,
        senderId: u8,
        receiverId: u8,
        message: *mut crate::LiteNetLib::Utils::INetSerializable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::Utils::NetDataWriter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::Utils::NetDataWriter = __cordl_object
            .invoke("WriteOne", (senderId, receiverId, message))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerAvatarChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerAvatarChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isConnectedOrConnecting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isConnectedOrConnecting", ())?;
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
    pub fn ResetLocalState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetLocalState", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerStateUpdate(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
        iPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerStateUpdate", (packet, iPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn HandleServerPlayerConnected(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
        iPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleServerPlayerConnected", (packet, iPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn WriteOneOnlyToFirstDegreeConnections(
        &mut self,
        senderId: u8,
        message: *mut crate::LiteNetLib::Utils::INetSerializable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::Utils::NetDataWriter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::Utils::NetDataWriter = __cordl_object
            .invoke("WriteOneOnlyToFirstDegreeConnections", (senderId, message))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterSerializer(
        &mut self,
        serializerType: crate::GlobalNamespace::ConnectedPlayerManager_MessageType,
        subSerializer: *mut INetworkPacketSubSerializer_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterSerializer", (serializerType, subSerializer))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveAllPlayers(
        &mut self,
        reason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAllPlayers", (reason))?;
        Ok(__cordl_ret)
    }
    pub fn SetLocalPlayerAvatar(
        &mut self,
        multiplayerAvatarsData: MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerAvatar", (multiplayerAvatarsData))?;
        Ok(__cordl_ret)
    }
    pub fn get_isConnecting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnecting", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerLatencyInitializedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerLatencyInitializedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_syncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_syncTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn Send<T>(
        &mut self,
        message: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (message))?;
        Ok(__cordl_ret)
    }
    pub fn SendImmediately(
        &mut self,
        message: *mut crate::LiteNetLib::Utils::INetSerializable,
        onlyFirstDegree: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendImmediately", (message, onlyFirstDegree))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePong(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PongPacket,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePong", (packet, player))?;
        Ok(__cordl_ret)
    }
    pub fn remove_connectionFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<ConnectionFailedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectionFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleDisconnected(
        &mut self,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDisconnected", (disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn InitializePlayerEncryption(
        &mut self,
        player: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializePlayerEncryption", (player))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerStateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerStateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn PollUpdate(
        &mut self,
        frameCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollUpdate", (frameCount))?;
        Ok(__cordl_ret)
    }
    pub fn SendImmediatelyToPlayer(
        &mut self,
        message: *mut crate::LiteNetLib::Utils::INetSerializable,
        toPlayer: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendImmediatelyToPlayer", (message, toPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn Log(
        &mut self,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (message))?;
        Ok(__cordl_ret)
    }
    pub fn HandleInitialized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddPlayer(
        &mut self,
        player: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPlayer", (player))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerStateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerStateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_connectedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IConnectedPlayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IConnectedPlayer = __cordl_object
            .invoke("get_localPlayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn FlushUnreliableQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushUnreliableQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_playerLatencyInitializedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerLatencyInitializedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SendToPlayer<T>(
        &mut self,
        message: T,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToPlayer", (message, player))?;
        Ok(__cordl_ret)
    }
    pub fn SetLocalPlayerSortIndex(
        &mut self,
        sortIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerSortIndex", (sortIndex))?;
        Ok(__cordl_ret)
    }
    pub fn add_connectedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetConnectionManager<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetConnectionManager", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNextConnectionId(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetNextConnectionId", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_playerConnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerConnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SendUnreliableEncryptedToPlayer<T>(
        &mut self,
        message: T,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendUnreliableEncryptedToPlayer", (message, player))?;
        Ok(__cordl_ret)
    }
    pub fn RemovePlayer(
        &mut self,
        player: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        reason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePlayer", (player, reason))?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayer_u8_0(
        &mut self,
        connectionId: u8,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer = __cordl_object
            .invoke("GetPlayer", (connectionId))?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayer_IConnection_u8_1(
        &mut self,
        connection: *mut IConnection,
        remoteConnectionId: u8,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer = __cordl_object
            .invoke("GetPlayer", (connection, remoteConnectionId))?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayer_String2(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer = __cordl_object
            .invoke("GetPlayer", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn add_disconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_disconnectedEvent", (value))?;
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
    pub fn remove_disconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_disconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SendImmediatelyFromPlayer(
        &mut self,
        message: *mut crate::LiteNetLib::Utils::INetSerializable,
        fromPlayer: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        onlyFirstDegree: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendImmediatelyFromPlayer",
                (message, fromPlayer, onlyFirstDegree),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        packet: *mut crate::LiteNetLib::Utils::INetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (writer, packet))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerIdentityUpdate(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
        iPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerIdentityUpdate", (packet, iPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn SendImmediatelyFromPlayerToPlayer(
        &mut self,
        message: *mut crate::LiteNetLib::Utils::INetSerializable,
        fromPlayer: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        toPlayer: *mut crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendImmediatelyFromPlayerToPlayer",
                (message, fromPlayer, toPlayer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleServerPlayerDisconnected(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket,
        iPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleServerPlayerDisconnected", (packet, iPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePing(
        &mut self,
        packet: *mut crate::GlobalNamespace::ConnectedPlayerManager_PingPacket,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePing", (packet, player))?;
        Ok(__cordl_ret)
    }
    pub fn New_IConnectionManager0(
        connectionManager: *mut IConnectionManager,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectionManager))?;
        Ok(__cordl_object)
    }
    pub fn New_ITimeProvider_ITaskUtility_IConnectionManager1(
        timeProvider: *mut crate::BGNet::Core::ITimeProvider,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
        connectionManager: *mut IConnectionManager,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (timeProvider, taskUtility, connectionManager))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl quest_hook::libil2cpp::ObjectType for ConnectedPlayerManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager+InternalMessageType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectedPlayerManager_InternalMessageType {
    KickPlayer = 8u8,
    MultiplayerSession = 7u8,
    Party = 6u8,
    Ping = 11u8,
    PlayerAvatarUpdate = 10u8,
    PlayerConnected = 1u8,
    PlayerDisconnected = 4u8,
    PlayerIdentity = 2u8,
    PlayerLatencyUpdate = 3u8,
    PlayerSortOrderUpdate = 5u8,
    PlayerStateUpdate = 9u8,
    Pong = 12u8,
    SyncTime = 0u8,
}
#[cfg(feature = "ConnectedPlayerManager+InternalMessageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_InternalMessageType => ""
    ."ConnectedPlayerManager/InternalMessageType"
);
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_KickPlayerPacket {
    __cordl_parent: crate::System::Object,
    pub disconnectedReason: DisconnectedReason,
}
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket => ""
    ."ConnectedPlayerManager/KickPlayerPacket"
);
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
impl crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket {
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
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
    pub fn Init(
        &mut self,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket = __cordl_object
            .invoke("Init", (disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager+MessageType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectedPlayerManager_MessageType {
    MultiplayerSession = 7u8,
    Party = 6u8,
}
#[cfg(feature = "ConnectedPlayerManager+MessageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_MessageType => ""
    ."ConnectedPlayerManager/MessageType"
);
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PingPacket {
    __cordl_parent: crate::System::Object,
    pub pingTime: i64,
}
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_PingPacket => ""
    ."ConnectedPlayerManager/PingPacket"
);
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerManager_PingPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConnectedPlayerManager_PingPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
impl crate::GlobalNamespace::ConnectedPlayerManager_PingPacket {
    pub fn Init(
        &mut self,
        pingTime: i64,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PingPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PingPacket = __cordl_object
            .invoke("Init", (pingTime))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
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
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_PingPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerAvatarPacket {
    __cordl_parent: crate::System::Object,
    pub playerAvatar: MultiplayerAvatarsData,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket => ""
    ."ConnectedPlayerManager/PlayerAvatarPacket"
);
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
impl crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket {
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
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
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        avatar: MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket = __cordl_object
            .invoke("Init", (avatar))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerConnectedPacket {
    __cordl_parent: crate::System::Object,
    pub remoteConnectionId: u8,
    pub userId: *mut crate::System::String,
    pub userName: *mut crate::System::String,
    pub isConnectionOwner: bool,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket => ""
    ."ConnectedPlayerManager/PlayerConnectedPacket"
);
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
impl crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket {
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
    pub fn Init(
        &mut self,
        connectionId: u8,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket = __cordl_object
            .invoke("Init", (connectionId, userId, userName, isConnectionOwner))?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerDisconnectedPacket {
    __cordl_parent: crate::System::Object,
    pub disconnectedReason: DisconnectedReason,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket => ""
    ."ConnectedPlayerManager/PlayerDisconnectedPacket"
);
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
impl crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket {
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
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket = __cordl_object
            .invoke("Init", (disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerIdentityPacket {
    __cordl_parent: crate::System::Object,
    pub playerState: PlayerStateHash,
    pub playerAvatar: MultiplayerAvatarsData,
    pub random: *mut ByteArrayNetSerializable,
    pub publicEncryptionKey: *mut ByteArrayNetSerializable,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket => ""
    ."ConnectedPlayerManager/PlayerIdentityPacket"
);
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
impl crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket {
    pub fn Init(
        &mut self,
        states: PlayerStateHash,
        avatar: MultiplayerAvatarsData,
        random: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        publicEncryptionKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket = __cordl_object
            .invoke("Init", (states, avatar, random, publicEncryptionKey))?;
        Ok(__cordl_ret)
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
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerSortOrderPacket {
    __cordl_parent: crate::System::Object,
    pub userId: *mut crate::System::String,
    pub sortIndex: i32,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket => ""
    ."ConnectedPlayerManager/PlayerSortOrderPacket"
);
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
impl crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket {
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
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        userId: *mut crate::System::String,
        sortIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket = __cordl_object
            .invoke("Init", (userId, sortIndex))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerStatePacket {
    __cordl_parent: crate::System::Object,
    pub playerState: PlayerStateHash,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket => ""
    ."ConnectedPlayerManager/PlayerStatePacket"
);
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
impl crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket {
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        states: PlayerStateHash,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket = __cordl_object
            .invoke("Init", (states))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PongPacket {
    __cordl_parent: crate::System::Object,
    pub pingTime: i64,
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_PongPacket => ""
    ."ConnectedPlayerManager/PongPacket"
);
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerManager_PongPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConnectedPlayerManager_PongPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
impl crate::GlobalNamespace::ConnectedPlayerManager_PongPacket {
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
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        pingTime: i64,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_PongPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_PongPacket = __cordl_object
            .invoke("Init", (pingTime))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_PongPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_SyncTimePacket {
    __cordl_parent: crate::System::Object,
    pub syncTime: i64,
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket => ""
    ."ConnectedPlayerManager/SyncTimePacket"
);
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
impl crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket {
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        syncTime: i64,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket = __cordl_object
            .invoke("Init", (syncTime))?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
