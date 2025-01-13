#[cfg(feature = "ConnectedPlayerManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub connectedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub initializedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub disconnectedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
    >,
    pub connectionFailedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::GlobalNamespace::ConnectionFailedReason>,
    >,
    pub playerConnectedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        >,
    >,
    pub playerDisconnectedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        >,
    >,
    pub playerStateChangedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        >,
    >,
    pub playerAvatarChangedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        >,
    >,
    pub playerOrderChangedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        >,
    >,
    pub playerLatencyInitializedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        >,
    >,
    pub syncTimeInitializedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _startTicks: i64,
    pub _syncTimeOffset: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LongRollingAverage,
    >,
    pub _timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
    pub _taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    pub _connectionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectionManager,
    >,
    pub _temporaryDataWriter: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::Utils::NetDataWriter,
    >,
    pub _temporaryEncryptedDataWriter: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::Utils::NetDataWriter,
    >,
    pub _reliableDataWriter: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::Utils::NetDataWriter,
    >,
    pub _unreliableDataWriter: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::Utils::NetDataWriter,
    >,
    pub _players: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
            >,
        >,
    >,
    pub _localPlayerState: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _localPlayerAvatars: crate::GlobalNamespace::MultiplayerAvatarsData,
    pub _encryptionKeys: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IDiffieHellmanKeyPair,
    >,
    pub _encryptionRandom: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub _localPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    >,
    pub _lastConnectionId: u8,
    pub _lastPollTime: i64,
    pub _lastPollFrame: i32,
    pub _lastPingTime: i64,
    pub _messageSerializer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NetworkPacketSerializer_2<
            crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        >,
    >,
}
#[cfg(feature = "ConnectedPlayerManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConnectedPlayerManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl crate::GlobalNamespace::ConnectedPlayerManager {
    pub const invalidSortIndex: i32 = -1i32;
    pub const kAllConnectionsId: u8 = 127u8;
    pub const kLocalConnectionId: u8 = 0u8;
    pub const kMaxUnreliableMessageLength: i32 = 412i32;
    pub const kPingUpdateFrequencyMs: i64 = 2000i64;
    pub const kTimeSensitiveAllowedReceiveWindowMs: i64 = 30i64;
    #[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
    pub type ConnectedPlayer = crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer;
    #[cfg(feature = "ConnectedPlayerManager+InternalMessageType")]
    pub type InternalMessageType = crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType;
    #[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
    pub type KickPlayerPacket = crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket;
    #[cfg(feature = "ConnectedPlayerManager+MessageType")]
    pub type MessageType = crate::GlobalNamespace::ConnectedPlayerManager_MessageType;
    #[cfg(feature = "ConnectedPlayerManager+PingPacket")]
    pub type PingPacket = crate::GlobalNamespace::ConnectedPlayerManager_PingPacket;
    #[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
    pub type PlayerAvatarPacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket;
    #[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
    pub type PlayerConnectedPacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket;
    #[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
    pub type PlayerDisconnectedPacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket;
    #[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
    pub type PlayerIdentityPacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket;
    #[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
    pub type PlayerSortOrderPacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket;
    #[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
    pub type PlayerStatePacket = crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket;
    #[cfg(feature = "ConnectedPlayerManager+PongPacket")]
    pub type PongPacket = crate::GlobalNamespace::ConnectedPlayerManager_PongPacket;
    #[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
    pub type SyncTimePacket = crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket;
    pub fn AddPlayer(
        &mut self,
        player: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPlayer", (player))?;
        Ok(__cordl_ret.into())
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
    pub fn FlushReliableQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushReliableQueue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushUnreliableQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushUnreliableQueue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConnectedPlayer(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        > = __cordl_object.invoke("GetConnectedPlayer", (index))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetNextConnectionId(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetNextConnectionId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayer_IConnection_u8_1(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        remoteConnectionId: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        > = __cordl_object.invoke("GetPlayer", (connection, remoteConnectionId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayer_Il2CppString2(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        > = __cordl_object.invoke("GetPlayer", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayer_u8_0(
        &mut self,
        connectionId: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        > = __cordl_object.invoke("GetPlayer", (connectionId))?;
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
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionDisconnected", (connection, disconnectedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleConnectionFailed(
        &mut self,
        reason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionFailed", (reason))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDisconnected(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDisconnected", (disconnectedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInitialized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleKickPlayerPacket(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket,
        >,
        iPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleKickPlayerPacket", (packet, iPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNetworkReceive(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNetworkReceive", (connection, reader, deliveryMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePing(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PingPacket,
        >,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePing", (packet, player))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerAvatarUpdate(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
        >,
        iPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerAvatarUpdate", (packet, iPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerIdentityUpdate(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
        >,
        iPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerIdentityUpdate", (packet, iPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerSortOrderUpdate(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket,
        >,
        iPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerSortOrderUpdate", (packet, iPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerStateUpdate(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
        >,
        iPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerStateUpdate", (packet, iPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePong(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PongPacket,
        >,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePong", (packet, player))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleServerPlayerConnected(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
        >,
        iPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleServerPlayerConnected", (packet, iPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleServerPlayerDisconnected(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket,
        >,
        iPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleServerPlayerDisconnected", (packet, iPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSyncTimePacket(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket,
        >,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSyncTimePacket", (packet, player))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializePlayerEncryption(
        &mut self,
        player: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializePlayerEncryption", (player))?;
        Ok(__cordl_ret.into())
    }
    pub fn KickPlayer(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KickPlayer", (userId, disconnectedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_IConnectionManager0(
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectionManager))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ITimeProvider_ITaskUtility_IConnectionManager1(
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (timeProvider, taskUtility, connectionManager))?;
        Ok(__cordl_object.into())
    }
    pub fn PollLateUpdateOptional(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollLateUpdateOptional", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn RegisterSerializer(
        &mut self,
        serializerType: crate::GlobalNamespace::ConnectedPlayerManager_MessageType,
        subSerializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::INetworkPacketSubSerializer_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterSerializer", (serializerType, subSerializer))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAllPlayers(
        &mut self,
        reason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAllPlayers", (reason))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePlayer(
        &mut self,
        player: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
        reason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePlayer", (player, reason))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetLocalState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetLocalState", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SendImmediately(
        &mut self,
        message: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
        onlyFirstDegree: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendImmediately", (message, onlyFirstDegree))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendImmediatelyExcludingPlayer(
        &mut self,
        message: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
        excludingPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn SendImmediatelyFromPlayer(
        &mut self,
        message: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
        fromPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn SendImmediatelyFromPlayerToPlayer(
        &mut self,
        message: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
        fromPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
        toPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendImmediatelyFromPlayerToPlayer",
                (message, fromPlayer, toPlayer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendImmediatelyFromPlayerToPlayerUnreliable<T>(
        &mut self,
        message: T,
        fromPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
        toPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn SendImmediatelyToPlayer(
        &mut self,
        message: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
        toPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendImmediatelyToPlayer", (message, toPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendToPlayer<T>(
        &mut self,
        message: T,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SendUnreliableEncryptedToPlayer<T>(
        &mut self,
        message: T,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
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
        Ok(__cordl_ret.into())
    }
    pub fn SendUnreliableFromPlayerToPlayer<T>(
        &mut self,
        message: T,
        fromPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        toPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerAvatar(
        &mut self,
        multiplayerAvatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerAvatar", (multiplayerAvatarsData))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerState(
        &mut self,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        setState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerState", (state, setState))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerSortIndex(
        &mut self,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        sortIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerSortIndex", (player, sortIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterSerializer(
        &mut self,
        serializerType: crate::GlobalNamespace::ConnectedPlayerManager_MessageType,
        subSerializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::INetworkPacketSubSerializer_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterSerializer", (serializerType, subSerializer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (writer, packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteOne(
        &mut self,
        senderId: u8,
        receiverId: u8,
        message: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetDataWriter,
        > = __cordl_object.invoke("WriteOne", (senderId, receiverId, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteOneEncrypted(
        &mut self,
        encryptionState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
        senderId: u8,
        receiverId: u8,
        message: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetDataWriter,
        > = __cordl_object
            .invoke(
                "WriteOneEncrypted",
                (encryptionState, senderId, receiverId, message),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteOneOnlyToFirstDegreeConnections(
        &mut self,
        senderId: u8,
        message: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetDataWriter,
        > = __cordl_object
            .invoke("WriteOneOnlyToFirstDegreeConnections", (senderId, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IConnectionManager0(
        &mut self,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectionManager))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ITimeProvider_ITaskUtility_IConnectionManager1(
        &mut self,
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (timeProvider, taskUtility, connectionManager))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_connectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_connectionFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::ConnectionFailedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectionFailedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_disconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_disconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_initializedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_initializedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_playerAvatarChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerAvatarChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_playerConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerConnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_playerDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_playerLatencyInitializedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerLatencyInitializedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_playerOrderChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerOrderChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_playerStateChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerStateChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_syncTimeInitializedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_syncTimeInitializedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_connectedPlayerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_connectedPlayerCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnectedOrConnecting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isConnectedOrConnecting", ())?;
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
    pub fn get_localPlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        > = __cordl_object.invoke("get_localPlayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_runTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_runTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_syncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_syncTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_syncTimeInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_syncTimeInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_connectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_connectionFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::ConnectionFailedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectionFailedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_disconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_disconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_initializedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_initializedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerAvatarChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerAvatarChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerConnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerLatencyInitializedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerLatencyInitializedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerOrderChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerOrderChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerStateChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerStateChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_syncTimeInitializedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_syncTimeInitializedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::ConnectedPlayerManager {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::ConnectedPlayerManager {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_ConnectedPlayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _isMe: bool,
    pub _isConnectionOwner: bool,
    pub _manager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ConnectedPlayerManager,
    >,
    pub _connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
    pub _parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
    >,
    pub _connectionId: u8,
    pub _remoteConnectionId: u8,
    pub _sortIndex: i32,
    pub _isConnected: bool,
    pub _disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    pub _isKicked: bool,
    pub _playerState: crate::GlobalNamespace::PlayerStateHash,
    pub _playerAvatars: crate::GlobalNamespace::MultiplayerAvatarsData,
    pub _publicEncryptionKey: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub _random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub _encryptionState: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
    >,
    pub _latency: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LongRollingAverage>,
}
#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/ConnectedPlayer";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CreateDirectlyConnectedPlayer(
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager,
        >,
        connectionId: u8,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateDirectlyConnectedPlayer",
                (manager, connectionId, connection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLocalPlayer(
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager,
        >,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
        publicEncryptionKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateLocalPlayer",
                (
                    manager,
                    userId,
                    userName,
                    isConnectionOwner,
                    publicEncryptionKey,
                    random,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateRemoteConnectedPlayer(
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager,
        >,
        connectionId: u8,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
        >,
        parent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateRemoteConnectedPlayer",
                (manager, connectionId, packet, parent),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerAvatarPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
        > = __cordl_object.invoke("GetPlayerAvatarPacket", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerConnectedPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
        > = __cordl_object.invoke("GetPlayerConnectedPacket", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerIdentityPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
        > = __cordl_object.invoke("GetPlayerIdentityPacket", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerSortOrderPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket,
        > = __cordl_object.invoke("GetPlayerSortOrderPacket", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerStatePacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
        > = __cordl_object.invoke("GetPlayerStatePacket", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasState(
        &mut self,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasState", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager,
        >,
        connectionId: u8,
        remoteConnectionId: u8,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        parent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
        isMe: bool,
        publicEncryptionKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn SetEncryptionState(
        &mut self,
        encryptionState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEncryptionState", (encryptionState))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKicked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKicked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerState(
        &mut self,
        playerState: crate::GlobalNamespace::PlayerStateHash,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerState", (playerState))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatar(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatar", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateIdentity(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIdentity", (packet))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSortIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UpdateSortIndex", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateState(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateState", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager,
        >,
        connectionId: u8,
        remoteConnectionId: u8,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        parent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer,
        >,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
        isMe: bool,
        publicEncryptionKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_connection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnection,
        > = __cordl_object.invoke("get_connection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_connectionId(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_connectionId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentLatency(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_currentLatency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disconnectedReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::DisconnectedReason> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::DisconnectedReason = __cordl_object
            .invoke("get_disconnectedReason", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_encryptionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        > = __cordl_object.invoke("get_encryptionState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasValidLatency(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasValidLatency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnectionOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnectionOwner", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDirectConnection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDirectConnection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isKicked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isKicked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isMe(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isMe", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplayerAvatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarsData = __cordl_object
            .invoke("get_multiplayerAvatarsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_offsetSyncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_offsetSyncTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_publicEncryptionKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_publicEncryptionKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_random(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_random", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_remoteConnectionId(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_remoteConnectionId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sortIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sortIndex", ())?;
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
    pub fn set_multiplayerAvatarsData(
        &mut self,
        value: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multiplayerAvatarsData", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
impl AsRef<crate::GlobalNamespace::IConnectedPlayer>
for crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer {
    fn as_ref(&self) -> &crate::GlobalNamespace::IConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+ConnectedPlayer")]
impl AsMut<crate::GlobalNamespace::IConnectedPlayer>
for crate::GlobalNamespace::ConnectedPlayerManager_ConnectedPlayer {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+InternalMessageType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectedPlayerManager_InternalMessageType {
    #[default]
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/InternalMessageType";
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
#[cfg(feature = "ConnectedPlayerManager+InternalMessageType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ConnectedPlayerManager+InternalMessageType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType {
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
#[cfg(feature = "ConnectedPlayerManager+InternalMessageType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType {
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
#[cfg(feature = "ConnectedPlayerManager+InternalMessageType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType {
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
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_KickPlayerPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
}
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/KickPlayerPacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket,
        > = __cordl_object.invoke("Init", (disconnectedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+KickPlayerPacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_KickPlayerPacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+MessageType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectedPlayerManager_MessageType {
    #[default]
    MultiplayerSession = 7u8,
    Party = 6u8,
}
#[cfg(feature = "ConnectedPlayerManager+MessageType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_MessageType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/MessageType";
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
#[cfg(feature = "ConnectedPlayerManager+MessageType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ConnectedPlayerManager_MessageType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ConnectedPlayerManager+MessageType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ConnectedPlayerManager_MessageType {
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
#[cfg(feature = "ConnectedPlayerManager+MessageType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ConnectedPlayerManager_MessageType {
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
#[cfg(feature = "ConnectedPlayerManager+MessageType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ConnectedPlayerManager_MessageType {
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
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PingPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub pingTime: i64,
}
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_PingPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/PingPacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerManager_PingPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        pingTime: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PingPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PingPacket,
        > = __cordl_object.invoke("Init", (pingTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PingPacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PingPacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PingPacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PingPacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PingPacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PingPacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PingPacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerAvatarPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub playerAvatar: crate::GlobalNamespace::MultiplayerAvatarsData,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/PlayerAvatarPacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        avatar: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
        > = __cordl_object.invoke("Init", (avatar))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerAvatarPacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerAvatarPacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerConnectedPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub remoteConnectionId: u8,
    pub userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub isConnectionOwner: bool,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/PlayerConnectedPacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        connectionId: u8,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
        > = __cordl_object
            .invoke("Init", (connectionId, userId, userName, isConnectionOwner))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerConnectedPacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerConnectedPacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerDisconnectedPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/PlayerDisconnectedPacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket,
        > = __cordl_object.invoke("Init", (disconnectedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerDisconnectedPacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerDisconnectedPacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerIdentityPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub playerState: crate::GlobalNamespace::PlayerStateHash,
    pub playerAvatar: crate::GlobalNamespace::MultiplayerAvatarsData,
    pub random: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ByteArrayNetSerializable,
    >,
    pub publicEncryptionKey: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ByteArrayNetSerializable,
    >,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/PlayerIdentityPacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        states: crate::GlobalNamespace::PlayerStateHash,
        avatar: crate::GlobalNamespace::MultiplayerAvatarsData,
        random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        publicEncryptionKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
        > = __cordl_object
            .invoke("Init", (states, avatar, random, publicEncryptionKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerIdentityPacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerIdentityPacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerSortOrderPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub sortIndex: i32,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/PlayerSortOrderPacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sortIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket,
        > = __cordl_object.invoke("Init", (userId, sortIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerSortOrderPacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerSortOrderPacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PlayerStatePacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub playerState: crate::GlobalNamespace::PlayerStateHash,
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/PlayerStatePacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
impl std::ops::Deref
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        states: crate::GlobalNamespace::PlayerStateHash,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
        > = __cordl_object.invoke("Init", (states))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PlayerStatePacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PlayerStatePacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_PongPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub pingTime: i64,
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_PongPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/PongPacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerManager_PongPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        pingTime: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PongPacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_PongPacket,
        > = __cordl_object.invoke("Init", (pingTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PongPacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_PongPacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PongPacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_PongPacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PongPacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+PongPacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_PongPacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager_SyncTimePacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub syncTime: i64,
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/SyncTimePacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        syncTime: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket,
        > = __cordl_object.invoke("Init", (syncTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ConnectedPlayerManager+SyncTimePacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ConnectedPlayerManager_SyncTimePacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
