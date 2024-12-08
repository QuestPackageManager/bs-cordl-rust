#[cfg(feature = "MultiplayerLobbyConnectionController+LobbyConnectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerLobbyConnectionController_LobbyConnectionState {
    Connected = 2i32,
    Connecting = 1i32,
    ConnectionFailed = 3i32,
    None = 0i32,
}
#[cfg(feature = "MultiplayerLobbyConnectionController+LobbyConnectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionState => ""
    ."MultiplayerLobbyConnectionController/LobbyConnectionState"
);
#[cfg(feature = "MultiplayerLobbyConnectionController+LobbyConnectionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerLobbyConnectionController_LobbyConnectionType {
    None = 0i32,
    PartyClient = 2i32,
    PartyHost = 1i32,
    QuickPlay = 3i32,
}
#[cfg(feature = "MultiplayerLobbyConnectionController+LobbyConnectionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType => ""
    ."MultiplayerLobbyConnectionController/LobbyConnectionType"
);
#[cfg(feature = "MultiplayerLobbyConnectionController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLobbyConnectionController {
    __cordl_parent: crate::System::Object,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _unifiedNetworkPlayerModel: *mut crate::GlobalNamespace::IUnifiedNetworkPlayerModel,
    pub connectionSuccessEvent: *mut crate::System::Action,
    pub connectionFailedEvent: *mut crate::System::Action_2<
        crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType,
        crate::GlobalNamespace::ConnectionFailedReason,
    >,
    pub _connectionState_k__BackingField: crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionState,
    pub _connectionType_k__BackingField: crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType,
    pub _connectionFailedReason_k__BackingField: crate::GlobalNamespace::ConnectionFailedReason,
    pub _partyConfig: *mut crate::GlobalNamespace::UnifiedNetworkPlayerModel_JoinMatchmakingPartyConfig,
    pub _retryAttemptsLeft: i32,
}
#[cfg(feature = "MultiplayerLobbyConnectionController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLobbyConnectionController => ""
    ."MultiplayerLobbyConnectionController"
);
#[cfg(feature = "MultiplayerLobbyConnectionController")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLobbyConnectionController {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyConnectionController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLobbyConnectionController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyConnectionController")]
impl crate::GlobalNamespace::MultiplayerLobbyConnectionController {
    #[cfg(feature = "MultiplayerLobbyConnectionController+LobbyConnectionState")]
    pub type LobbyConnectionState = crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionState;
    #[cfg(feature = "MultiplayerLobbyConnectionController+LobbyConnectionType")]
    pub type LobbyConnectionType = crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType;
    pub fn ClearCurrentConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCurrentConnection", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConnectToMatchmaking(
        &mut self,
        beatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
        songPackMask: crate::GlobalNamespace::SongPackMask,
        allowSongSelection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ConnectToMatchmaking",
                (beatmapDifficultyMask, songPackMask, allowSongSelection),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ConnectToParty(
        &mut self,
        serverCode: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectToParty", (serverCode))?;
        Ok(__cordl_ret)
    }
    pub fn ConnectToServer(
        &mut self,
        server: *mut crate::GlobalNamespace::INetworkPlayer,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectToServer", (server, password))?;
        Ok(__cordl_ret)
    }
    pub fn CreateOrConnectToDestinationParty(
        &mut self,
        lobbyDestination: *mut crate::GlobalNamespace::SelectMultiplayerLobbyDestination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateOrConnectToDestinationParty", (lobbyDestination))?;
        Ok(__cordl_ret)
    }
    pub fn CreateParty(
        &mut self,
        data: crate::GlobalNamespace::CreateServerFormData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateParty", (data))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerSessionManagerConnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplayerSessionManagerConnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerSessionManagerConnectionFailed(
        &mut self,
        reason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplayerSessionManagerConnectionFailed", (reason))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerSessionManagerConnectionFailedWithRetry(
        &mut self,
        reason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerSessionManagerConnectionFailedWithRetry",
                (reason),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LeaveLobby(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LeaveLobby", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn add_connectionFailedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType,
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
    pub fn add_connectionSuccessEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectionSuccessEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_connectionFailedReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ConnectionFailedReason> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ConnectionFailedReason = __cordl_object
            .invoke("get_connectionFailedReason", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_connectionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionState = __cordl_object
            .invoke("get_connectionState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_connectionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType = __cordl_object
            .invoke("get_connectionType", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_connectionFailedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType,
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
    pub fn remove_connectionSuccessEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectionSuccessEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_connectionFailedReason(
        &mut self,
        value: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_connectionFailedReason", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_connectionState(
        &mut self,
        value: crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_connectionState", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_connectionType(
        &mut self,
        value: crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_connectionType", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerLobbyConnectionController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLobbyConnectionController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
