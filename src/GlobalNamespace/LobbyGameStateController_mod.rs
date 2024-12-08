#[cfg(feature = "LobbyGameStateController")]
#[repr(C)]
#[derive(Debug)]
pub struct LobbyGameStateController {
    __cordl_parent: crate::System::Object,
    pub _lobbyPlayersDataModel: *mut ILobbyPlayersDataModel,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _menuRpcManager: *mut IMenuRpcManager,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _multiplayerLevelLoader: *mut MultiplayerLevelLoader,
    pub _menuTransitionsHelper: *mut MenuTransitionsHelper,
    pub _lobbyGameStateModel: *mut LobbyGameStateModel,
    pub _lobbyPlayerPermissionsModel: *mut LobbyPlayerPermissionsModel,
    pub _beatmapCharacteristicCollection: *mut BeatmapCharacteristicCollection,
    pub _beatmapLevelsModel: *mut BeatmapLevelsModel,
    pub selectedLevelGameplaySetupDataChangedEvent: *mut crate::System::Action_1<
        *mut ILevelGameplaySetupData,
    >,
    pub gameStartedEvent: *mut crate::System::Action_1<*mut ILevelGameplaySetupData>,
    pub gameStartCancelledEvent: *mut crate::System::Action,
    pub countdownStartedEvent: *mut crate::System::Action,
    pub countdownCancelledEvent: *mut crate::System::Action,
    pub songStillDownloadingEvent: *mut crate::System::Action,
    pub startTimeChangedEvent: *mut crate::System::Action,
    pub levelFinishedEvent: *mut crate::System::Action_2<
        *mut MultiplayerLevelScenesTransitionSetupDataSO,
        *mut MultiplayerResultsData,
    >,
    pub levelDidGetDisconnectedEvent: *mut crate::System::Action_1<DisconnectedReason>,
    pub lobbyDisconnectedEvent: *mut crate::System::Action,
    pub beforeSceneSwitchCallbackEvent: *mut crate::System::Action,
    pub lobbyStateChangedEvent: *mut crate::System::Action_1<MultiplayerLobbyState>,
    pub startButtonEnabledEvent: *mut crate::System::Action_1<CannotStartGameReason>,
    pub playerMissingEntitlementsChangedEvent: *mut crate::System::Action_1<
        *mut PlayersMissingEntitlementsNetSerializable,
    >,
    pub _levelStartInitiated_k__BackingField: bool,
    pub _countdownStarted_k__BackingField: bool,
    pub _countdownEndTime_k__BackingField: i64,
    pub _isDisconnected_k__BackingField: bool,
    pub _disconnectedReason_k__BackingField: DisconnectedReason,
    pub _predictedStartTime: i64,
    pub _startTime: i64,
    pub _levelStartedOnTime: bool,
    pub _state: MultiplayerLobbyState,
    pub _cannotStartGameReason: CannotStartGameReason,
    pub _selectedLevelGameplaySetupData: *mut LevelGameplaySetupData,
}
#[cfg(feature = "LobbyGameStateController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LobbyGameStateController => ""
    ."LobbyGameStateController"
);
#[cfg(feature = "LobbyGameStateController")]
impl std::ops::Deref for LobbyGameStateController {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl std::ops::DerefMut for LobbyGameStateController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl LobbyGameStateController {
    pub const kLongTimerMs: i64 = 60000i64;
    pub const kShortTimerMs: i64 = 5000i64;
    #[cfg(feature = "LobbyGameStateController+__c")]
    pub type __c = crate::GlobalNamespace::LobbyGameStateController___c;
    #[cfg(feature = "LobbyGameStateController+__c__DisplayClass98_0")]
    pub type __c__DisplayClass98_0 = crate::GlobalNamespace::LobbyGameStateController___c__DisplayClass98_0;
    pub fn remove_startTimeChangedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startTimeChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Activate", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_levelDidGetDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelDidGetDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerMissingEntitlementsChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut PlayersMissingEntitlementsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerMissingEntitlementsChangedEvent", (value))?;
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
    pub fn get_state(&mut self) -> quest_hook::libil2cpp::Result<MultiplayerLobbyState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: MultiplayerLobbyState = __cordl_object.invoke("get_state", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_selectedLevelGameplaySetupDataChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ILevelGameplaySetupData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectedLevelGameplaySetupDataChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_countdownStartedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_countdownStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_countdownEndTime(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_countdownEndTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerSessionManagerConnectionOwnerStateChanged(
        &mut self,
        connectedPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerSessionManagerConnectionOwnerStateChanged",
                (connectedPlayer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartMultiplayerLevel(
        &mut self,
        gameplaySetupData: *mut ILevelGameplaySetupData,
        beatmapLevelData: *mut IBeatmapLevelData,
        beforeSceneSwitchCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartMultiplayerLevel",
                (gameplaySetupData, beatmapLevelData, beforeSceneSwitchCallback),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_gameStartedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ILevelGameplaySetupData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_countdownStarted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_countdownStarted", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerSetStartGameTime(
        &mut self,
        userId: *mut crate::System::String,
        startTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerSetStartGameTime", (userId, startTime))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerSetPlayersMissingEntitlementsToLevel(
        &mut self,
        userId: *mut crate::System::String,
        playersMissingEntitlements: *mut PlayersMissingEntitlementsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerSetPlayersMissingEntitlementsToLevel",
                (userId, playersMissingEntitlements),
            )?;
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
    pub fn remove_countdownCancelledEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_countdownCancelledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_countdownEndTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_countdownEndTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_playerMissingEntitlementsChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut PlayersMissingEntitlementsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerMissingEntitlementsChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_lobbyStateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<MultiplayerLobbyState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_lobbyStateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerSetCountdownEndTime(
        &mut self,
        userId: *mut crate::System::String,
        countdownTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerSetCountdownEndTime", (userId, countdownTime))?;
        Ok(__cordl_ret)
    }
    pub fn add_beforeSceneSwitchCallbackEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beforeSceneSwitchCallbackEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerClearSelectedGameplayModifiers(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerClearSelectedGameplayModifiers", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn set_countdownStarted(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_countdownStarted", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_levelFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerLevelScenesTransitionSetupDataSO,
            *mut MultiplayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_songStillDownloadingEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_songStillDownloadingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSetIsStartButtonEnabled(
        &mut self,
        userId: *mut crate::System::String,
        cannotStartGameReason: CannotStartGameReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSetIsStartButtonEnabled", (userId, cannotStartGameReason))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerClearSelectedBeatmap(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerClearSelectedBeatmap", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLevelLoaderCountdownFinished(
        &mut self,
        gameplaySetupData: *mut ILevelGameplaySetupData,
        beatmapLevelData: *mut IBeatmapLevelData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelLoaderCountdownFinished",
                (gameplaySetupData, beatmapLevelData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PredictCountdownEndTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PredictCountdownEndTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_lobbyDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_lobbyDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_lobbyDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_lobbyDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerCancelledLevelStart(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerCancelledLevelStart", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn remove_gameStartedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ILevelGameplaySetupData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_levelStartInitiated(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelStartInitiated", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerStartedLevel(
        &mut self,
        userId: *mut crate::System::String,
        beatmapKeyNetSerializable: *mut BeatmapKeyNetSerializable,
        gameplayModifiers: *mut GameplayModifiers,
        startTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerStartedLevel",
                (userId, beatmapKeyNetSerializable, gameplayModifiers, startTime),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLevelDidDisconnect(
        &mut self,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplayerLevelDidDisconnect", (disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn add_levelDidGetDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelDidGetDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isDisconnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDisconnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_beforeSceneSwitchCallbackEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beforeSceneSwitchCallbackEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_gameStartCancelledEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameStartCancelledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_startTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_startTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_startButtonEnabledEvent(
        &mut self,
        value: *mut crate::System::Action_1<CannotStartGameReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startButtonEnabledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedLevelGameplaySetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut ILevelGameplaySetupData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ILevelGameplaySetupData = __cordl_object
            .invoke("get_selectedLevelGameplaySetupData", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectedLevelGameplaySetupDataChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ILevelGameplaySetupData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectedLevelGameplaySetupDataChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_state(
        &mut self,
        value: MultiplayerLobbyState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_state", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleStartTimeChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleStartTimeChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerSetSelectedGameplayModifiers(
        &mut self,
        userId: *mut crate::System::String,
        modifiers: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerSetSelectedGameplayModifiers",
                (userId, modifiers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_countdownStartedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_countdownStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_levelFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerLevelScenesTransitionSetupDataSO,
            *mut MultiplayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_disconnectedReason(
        &mut self,
        value: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disconnectedReason", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_levelStartInitiated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_levelStartInitiated", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGameStateAndConfigurationAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("GetGameStateAndConfigurationAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn set_startTime(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerSessionManagerDisconnected(
        &mut self,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerSessionManagerDisconnected",
                (disconnectedReason),
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
    pub fn get_predictedCountdownEndTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_predictedCountdownEndTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cannotStartGameReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<CannotStartGameReason> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: CannotStartGameReason = __cordl_object
            .invoke("get_cannotStartGameReason", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopListeningToGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopListeningToGameStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_songStillDownloadingEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_songStillDownloadingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLevelLoaderStillDownloadingSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplayerLevelLoaderStillDownloadingSong", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLevelDidFinish(
        &mut self,
        multiplayerLevelScenesTransitionSetupData: *mut MultiplayerLevelScenesTransitionSetupDataSO,
        multiplayerResultsData: *mut MultiplayerResultsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelDidFinish",
                (multiplayerLevelScenesTransitionSetupData, multiplayerResultsData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_countdownCancelledEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_countdownCancelledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerSetSelectedBeatmap(
        &mut self,
        userId: *mut crate::System::String,
        beatmapKeySerializable: *mut BeatmapKeyNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerSetSelectedBeatmap",
                (userId, beatmapKeySerializable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_gameStartCancelledEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameStartCancelledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_startButtonEnabledEvent(
        &mut self,
        value: *mut crate::System::Action_1<CannotStartGameReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startButtonEnabledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isDisconnected(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDisconnected", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ClearDisconnectedState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDisconnectedState", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartListeningToGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartListeningToGameStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsCloseToStartGame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCloseToStartGame", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopLoading", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentLevelIfGameStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCurrentLevelIfGameStarted", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerCancelCountdown(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerCancelCountdown", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn add_lobbyStateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<MultiplayerLobbyState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_lobbyStateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_startTimeChangedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startTimeChangedEvent", (value))?;
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
#[cfg(feature = "LobbyGameStateController")]
impl quest_hook::libil2cpp::ObjectType for LobbyGameStateController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
