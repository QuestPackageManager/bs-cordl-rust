#[cfg(feature = "MultiplayerSessionManager+ConnectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerSessionManager_ConnectionState {
    Connected = 2i32,
    Connecting = 1i32,
    Disconnected = 0i32,
    Disconnecting = 3i32,
}
#[cfg(feature = "MultiplayerSessionManager+ConnectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerSessionManager_ConnectionState => ""
    ."MultiplayerSessionManager/ConnectionState"
);
#[cfg(feature = "MultiplayerSessionManager+MessageType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerSessionManager_MessageType {
    GameplayRpc = 1u8,
    MenuRpc = 0u8,
    NodePoseSyncState = 2u8,
    NodePoseSyncStateDelta = 4u8,
    OptionalAvatarData = 6u8,
    ScoreSyncState = 3u8,
    ScoreSyncStateDelta = 5u8,
}
#[cfg(feature = "MultiplayerSessionManager+MessageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerSessionManager_MessageType => ""
    ."MultiplayerSessionManager/MessageType"
);
#[cfg(feature = "MultiplayerSessionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerSessionManager {
    __cordl_parent: crate::GlobalNamespace::StandaloneMonobehavior,
    pub _packetSerializer: *mut crate::GlobalNamespace::NetworkPacketSerializer_2<
        crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
        *mut crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub _connectedPlayers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub _localPlayerState: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _maxPlayerCount: i32,
    pub _connectionState: crate::GlobalNamespace::MultiplayerSessionManager_ConnectionState,
    pub _freeSortIndices: *mut crate::System::Collections::Generic::Queue_1<i32>,
    pub connectedEvent: *mut crate::System::Action,
    pub connectionFailedEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::ConnectionFailedReason,
    >,
    pub playerConnectedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub playerDisconnectedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub playerAvatarChangedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub playerStateChangedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub connectionOwnerStateChangedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub disconnectedEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::DisconnectedReason,
    >,
    pub pollUpdateEvent: *mut crate::System::Action,
    pub _connectionOwner_k__BackingField: *mut crate::GlobalNamespace::IConnectedPlayer,
    pub _connectedPlayerManager: *mut crate::GlobalNamespace::ConnectedPlayerManager,
}
#[cfg(feature = "MultiplayerSessionManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerSessionManager => ""
    ."MultiplayerSessionManager"
);
#[cfg(feature = "MultiplayerSessionManager")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerSessionManager {
    type Target = crate::GlobalNamespace::StandaloneMonobehavior;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerSessionManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerSessionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerSessionManager")]
impl crate::GlobalNamespace::MultiplayerSessionManager {
    pub const kMultiplayerSessionState: &'static str = "multiplayer_session";
    #[cfg(feature = "MultiplayerSessionManager+ConnectionState")]
    pub type ConnectionState = crate::GlobalNamespace::MultiplayerSessionManager_ConnectionState;
    #[cfg(feature = "MultiplayerSessionManager+MessageType")]
    pub type MessageType = crate::GlobalNamespace::MultiplayerSessionManager_MessageType;
    #[cfg(feature = "MultiplayerSessionManager+SessionType")]
    pub type SessionType = crate::GlobalNamespace::MultiplayerSessionManager_SessionType;
    #[cfg(feature = "MultiplayerSessionManager+__c")]
    pub type __c = crate::GlobalNamespace::MultiplayerSessionManager___c;
    #[cfg(feature = "MultiplayerSessionManager+__c__DisplayClass98_0")]
    pub type __c__DisplayClass98_0 = crate::GlobalNamespace::MultiplayerSessionManager___c__DisplayClass98_0;
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
    pub fn EndSession(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndSession", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetConnectedPlayer(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::IConnectedPlayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IConnectedPlayer = __cordl_object
            .invoke("GetConnectedPlayer", (i))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextAvailableSortIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNextAvailableSortIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayerByUserId(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::IConnectedPlayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IConnectedPlayer = __cordl_object
            .invoke("GetPlayerByUserId", (userId))?;
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
        reason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectionFailed", (reason))?;
        Ok(__cordl_ret)
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
    pub fn HandlePlayerAvatarChanged(
        &mut self,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerAvatarChanged", (player))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerConnected(
        &mut self,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerConnected", (player))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerDisconnected(
        &mut self,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerDisconnected", (player))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerLatencyInitialized(
        &mut self,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerLatencyInitialized", (player))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerOrderChanged(
        &mut self,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerOrderChanged", (player))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerStateChanged(
        &mut self,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerStateChanged", (player))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSyncTimeInitialized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSyncTimeInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn KickPlayer(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KickPlayer", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn LocalPlayerHasState(
        &mut self,
        state: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LocalPlayerHasState", (state))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnApplicationPause(
        &mut self,
        pauseStatus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationPause", (pauseStatus))?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback<T>(
        &mut self,
        serializerType: crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
        callback: *mut crate::System::Action_2<
            T,
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
        constructor: *mut crate::System::Func_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (serializerType, callback, constructor))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterSerializer(
        &mut self,
        serializerType: crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
        subSerializer: *mut crate::GlobalNamespace::INetworkPacketSubSerializer_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterSerializer", (serializerType, subSerializer))?;
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
    pub fn SendToPlayer<T>(
        &mut self,
        message: T,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
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
    pub fn SendUnreliableEncryptedToPlayer<T>(
        &mut self,
        message: T,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
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
    pub fn SendUnreliableFromPlayerToPlayer<T>(
        &mut self,
        message: T,
        fromPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
        toPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
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
    pub fn SetLocalPlayerState(
        &mut self,
        state: *mut crate::System::String,
        hasState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerState", (state, hasState))?;
        Ok(__cordl_ret)
    }
    pub fn SetMaxPlayerCount(
        &mut self,
        maxPlayerCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMaxPlayerCount", (maxPlayerCount))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartSession(
        &mut self,
        sessionType: crate::GlobalNamespace::MultiplayerSessionManager_SessionType,
        connectedPlayerManager: *mut crate::GlobalNamespace::ConnectedPlayerManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSession", (sessionType, connectedPlayerManager))?;
        Ok(__cordl_ret)
    }
    pub fn TryUpdateConnectedPlayer(
        &mut self,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
        isPlayerConnected: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryUpdateConnectedPlayer", (player, isPlayerConnected))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterCallback<T>(
        &mut self,
        serializerType: crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallback", (serializerType))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterSerializer(
        &mut self,
        serializerType: crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
        subSerializer: *mut crate::GlobalNamespace::INetworkPacketSubSerializer_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterSerializer", (serializerType, subSerializer))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateConnectionState(
        &mut self,
        updateReason: crate::GlobalNamespace::UpdateConnectionStateReason,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
        connectionFailedReason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateConnectionState",
                (updateReason, disconnectedReason, connectionFailedReason),
            )?;
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
    pub fn add_connectionFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::ConnectionFailedReason,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectionFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_connectionOwnerStateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectionOwnerStateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_disconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_disconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerAvatarChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerAvatarChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerConnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerConnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerStateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerStateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_pollUpdateEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_pollUpdateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_connectedPlayerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_connectedPlayerCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_connectedPlayerManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ConnectedPlayerManager,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ConnectedPlayerManager = __cordl_object
            .invoke("get_connectedPlayerManager", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_connectedPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        > = __cordl_object.invoke("get_connectedPlayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_connectionOwner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::IConnectedPlayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IConnectedPlayer = __cordl_object
            .invoke("get_connectionOwner", ())?;
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
    pub fn get_isConnectingOrConnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isConnectingOrConnected", ())?;
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
    pub fn get_isSpectating(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSpectating", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isSyncTimeInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSyncTimeInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::IConnectedPlayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IConnectedPlayer = __cordl_object
            .invoke("get_localPlayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxPlayerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxPlayerCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_syncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_syncTime", ())?;
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
    pub fn remove_connectionFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::ConnectionFailedReason,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectionFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_connectionOwnerStateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectionOwnerStateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_disconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_disconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerAvatarChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerAvatarChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerConnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerConnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerStateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerStateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_pollUpdateEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_pollUpdateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_connectionOwner(
        &mut self,
        value: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_connectionOwner", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerSessionManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerSessionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerSessionManager+SessionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerSessionManager_SessionType {
    DedicatedServer = 2i32,
    Player = 0i32,
    Spectator = 1i32,
}
#[cfg(feature = "MultiplayerSessionManager+SessionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerSessionManager_SessionType => ""
    ."MultiplayerSessionManager/SessionType"
);
