#[cfg(feature = "UnifiedNetworkPlayerModel+ActiveNetworkPlayerModelType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnifiedNetworkPlayerModel_ActiveNetworkPlayerModelType {
    GameLift = 0i32,
    Platform = 1i32,
}
#[cfg(feature = "UnifiedNetworkPlayerModel+ActiveNetworkPlayerModelType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::UnifiedNetworkPlayerModel_ActiveNetworkPlayerModelType => ""
    ."UnifiedNetworkPlayerModel/ActiveNetworkPlayerModelType"
);
#[cfg(feature = "UnifiedNetworkPlayerModel+JoinMatchmakingPartyConfig")]
#[repr(C)]
#[derive(Debug)]
pub struct UnifiedNetworkPlayerModel_JoinMatchmakingPartyConfig {
    __cordl_parent: crate::System::Object,
    pub selectionMask: BeatmapLevelSelectionMask,
    pub configuration: GameplayServerConfiguration,
    pub secret: *mut crate::System::String,
    pub code: *mut crate::System::String,
}
#[cfg(feature = "UnifiedNetworkPlayerModel+JoinMatchmakingPartyConfig")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::UnifiedNetworkPlayerModel_JoinMatchmakingPartyConfig => ""
    ."UnifiedNetworkPlayerModel/JoinMatchmakingPartyConfig"
);
#[cfg(feature = "UnifiedNetworkPlayerModel+JoinMatchmakingPartyConfig")]
impl std::ops::Deref
for crate::GlobalNamespace::UnifiedNetworkPlayerModel_JoinMatchmakingPartyConfig {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnifiedNetworkPlayerModel+JoinMatchmakingPartyConfig")]
impl std::ops::DerefMut
for crate::GlobalNamespace::UnifiedNetworkPlayerModel_JoinMatchmakingPartyConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnifiedNetworkPlayerModel+JoinMatchmakingPartyConfig")]
impl crate::GlobalNamespace::UnifiedNetworkPlayerModel_JoinMatchmakingPartyConfig {
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
#[cfg(feature = "UnifiedNetworkPlayerModel+JoinMatchmakingPartyConfig")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UnifiedNetworkPlayerModel_JoinMatchmakingPartyConfig {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnifiedNetworkPlayerModel+StartClientPartyConfig")]
#[repr(C)]
#[derive(Debug)]
pub struct UnifiedNetworkPlayerModel_StartClientPartyConfig {
    __cordl_parent: crate::System::Object,
    pub selectionMask: BeatmapLevelSelectionMask,
    pub configuration: GameplayServerConfiguration,
}
#[cfg(feature = "UnifiedNetworkPlayerModel+StartClientPartyConfig")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::UnifiedNetworkPlayerModel_StartClientPartyConfig => ""
    ."UnifiedNetworkPlayerModel/StartClientPartyConfig"
);
#[cfg(feature = "UnifiedNetworkPlayerModel+StartClientPartyConfig")]
impl std::ops::Deref
for crate::GlobalNamespace::UnifiedNetworkPlayerModel_StartClientPartyConfig {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnifiedNetworkPlayerModel+StartClientPartyConfig")]
impl std::ops::DerefMut
for crate::GlobalNamespace::UnifiedNetworkPlayerModel_StartClientPartyConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnifiedNetworkPlayerModel+StartClientPartyConfig")]
impl crate::GlobalNamespace::UnifiedNetworkPlayerModel_StartClientPartyConfig {
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
#[cfg(feature = "UnifiedNetworkPlayerModel+StartClientPartyConfig")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UnifiedNetworkPlayerModel_StartClientPartyConfig {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnifiedNetworkPlayerModel")]
#[repr(C)]
#[derive(Debug)]
pub struct UnifiedNetworkPlayerModel {
    __cordl_parent: crate::System::Object,
    pub _gameLiftNetworkPlayerModel: *mut GameLiftNetworkPlayerModel,
    pub _platformNetworkPlayerModel: *mut PlatformNetworkPlayerModel,
    pub connectedPlayerManagerCreatedEvent: *mut crate::System::Action_1<
        *mut INetworkPlayerModel,
    >,
    pub connectedPlayerManagerDestroyedEvent: *mut crate::System::Action_1<
        *mut INetworkPlayerModel,
    >,
    pub partySizeChangedEvent: *mut crate::System::Action_1<i32>,
    pub partyRefreshingEvent: *mut crate::System::Action,
    pub partyChangedEvent: *mut crate::System::Action_1<*mut INetworkPlayerModel>,
    pub joinRequestedEvent: *mut crate::System::Action_1<*mut INetworkPlayer>,
    pub inviteRequestedEvent: *mut crate::System::Action_1<*mut INetworkPlayer>,
    pub _partyMessageHandler: *mut PartyMessageHandler,
    pub _friendPartyMessageHandler: *mut PartyMessageHandler,
    pub _activeNetworkPlayerModelType: crate::GlobalNamespace::UnifiedNetworkPlayerModel_ActiveNetworkPlayerModelType,
}
#[cfg(feature = "UnifiedNetworkPlayerModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for UnifiedNetworkPlayerModel => ""
    ."UnifiedNetworkPlayerModel"
);
#[cfg(feature = "UnifiedNetworkPlayerModel")]
impl std::ops::Deref for UnifiedNetworkPlayerModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnifiedNetworkPlayerModel")]
impl std::ops::DerefMut for UnifiedNetworkPlayerModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnifiedNetworkPlayerModel")]
impl UnifiedNetworkPlayerModel {
    #[cfg(feature = "UnifiedNetworkPlayerModel+ActiveNetworkPlayerModelType")]
    pub type ActiveNetworkPlayerModelType = crate::GlobalNamespace::UnifiedNetworkPlayerModel_ActiveNetworkPlayerModelType;
    #[cfg(feature = "UnifiedNetworkPlayerModel+_get_otherPlayers_d__64")]
    pub type _get_otherPlayers_d__64 = crate::GlobalNamespace::UnifiedNetworkPlayerModel__get_otherPlayers_d__64;
    #[cfg(feature = "UnifiedNetworkPlayerModel+JoinMatchmakingPartyConfig")]
    pub type JoinMatchmakingPartyConfig = crate::GlobalNamespace::UnifiedNetworkPlayerModel_JoinMatchmakingPartyConfig;
    #[cfg(feature = "UnifiedNetworkPlayerModel+StartClientPartyConfig")]
    pub type StartClientPartyConfig = crate::GlobalNamespace::UnifiedNetworkPlayerModel_StartClientPartyConfig;
    pub fn CreatePartyConnection<T>(
        &mut self,
        partyConfig: *mut INetworkPlayerModelPartyConfig_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CreatePartyConnection", (partyConfig))?;
        Ok(__cordl_ret)
    }
    pub fn DestroyPartyConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyPartyConnection", ())?;
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
    pub fn HandleConnectedPlayerManagerCreated(
        &mut self,
        networkPlayerModel: *mut INetworkPlayerModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectedPlayerManagerCreated", (networkPlayerModel))?;
        Ok(__cordl_ret)
    }
    pub fn HandleConnectedPlayerManagerDestroyed(
        &mut self,
        networkPlayerModel: *mut INetworkPlayerModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectedPlayerManagerDestroyed", (networkPlayerModel))?;
        Ok(__cordl_ret)
    }
    pub fn HandleFriendConnectToMasterServer(
        &mut self,
        secret: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFriendConnectToMasterServer", (secret))?;
        Ok(__cordl_ret)
    }
    pub fn HandleFriendConnected(
        &mut self,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFriendConnected", (player))?;
        Ok(__cordl_ret)
    }
    pub fn HandleInviteRequested(
        &mut self,
        player: *mut INetworkPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInviteRequested", (player))?;
        Ok(__cordl_ret)
    }
    pub fn HandleJoinRequested(
        &mut self,
        player: *mut INetworkPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleJoinRequested", (player))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePartyChanged(
        &mut self,
        playerModel: *mut INetworkPlayerModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePartyChanged", (playerModel))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePartyRefreshing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePartyRefreshing", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePartySizeChanged(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePartySizeChanged", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayersChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayersChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RefreshAlternateDiscoveryModels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAlternateDiscoveryModels", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetMasterServerReachability(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetMasterServerReachability", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetActiveNetworkPlayerModelType(
        &mut self,
        activeNetworkPlayerModelType: crate::GlobalNamespace::UnifiedNetworkPlayerModel_ActiveNetworkPlayerModelType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActiveNetworkPlayerModelType", (activeNetworkPlayerModelType))?;
        Ok(__cordl_ret)
    }
    pub fn SetServerFilter(
        &mut self,
        selectionMask: BeatmapLevelSelectionMask,
        configuration: GameplayServerConfiguration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetServerFilter", (selectionMask, configuration))?;
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
    pub fn add_connectedPlayerManagerCreatedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayerModel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerManagerCreatedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_connectedPlayerManagerDestroyedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayerModel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectedPlayerManagerDestroyedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_inviteRequestedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_inviteRequestedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_joinRequestedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_joinRequestedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_partyChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayerModel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_partyChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_partyRefreshingEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_partyRefreshingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_partySizeChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_partySizeChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_activeNetworkPlayerModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut INetworkPlayerModel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut INetworkPlayerModel = __cordl_object
            .invoke("get_activeNetworkPlayerModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_activeNetworkPlayerModelType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::UnifiedNetworkPlayerModel_ActiveNetworkPlayerModelType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::UnifiedNetworkPlayerModel_ActiveNetworkPlayerModelType = __cordl_object
            .invoke("get_activeNetworkPlayerModelType", ())?;
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
    pub fn get_connectedPlayerManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut ConnectedPlayerManager> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ConnectedPlayerManager = __cordl_object
            .invoke("get_connectedPlayerManager", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentPartySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_currentPartySize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_discoveryEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_discoveryEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enableFriends(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableFriends", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_friends(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut INetworkPlayer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut INetworkPlayer,
        > = __cordl_object.invoke("get_friends", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasNetworkingFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasNetworkingFailed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayerIsPartyOwner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_localPlayerIsPartyOwner", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_otherPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut INetworkPlayer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut INetworkPlayer,
        > = __cordl_object.invoke("get_otherPlayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_partyPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut INetworkPlayer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut INetworkPlayer,
        > = __cordl_object.invoke("get_partyPlayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_publicServers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut INetworkPlayer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut INetworkPlayer,
        > = __cordl_object.invoke("get_publicServers", ())?;
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
    pub fn remove_connectedPlayerManagerCreatedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayerModel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerManagerCreatedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_connectedPlayerManagerDestroyedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayerModel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectedPlayerManagerDestroyedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_inviteRequestedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_inviteRequestedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_joinRequestedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_joinRequestedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_partyChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayerModel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_partyChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_partyRefreshingEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_partyRefreshingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_partySizeChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_partySizeChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_discoveryEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_discoveryEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_enableFriends(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableFriends", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnifiedNetworkPlayerModel")]
impl quest_hook::libil2cpp::ObjectType for UnifiedNetworkPlayerModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
