#[cfg(feature = "LobbyGameStateController")]
#[repr(C)]
#[derive(Debug)]
pub struct LobbyGameStateController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lobbyPlayersDataModel: *mut crate::GlobalNamespace::ILobbyPlayersDataModel,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _menuRpcManager: *mut crate::GlobalNamespace::IMenuRpcManager,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _multiplayerLevelLoader: *mut crate::GlobalNamespace::MultiplayerLevelLoader,
    pub _menuTransitionsHelper: *mut crate::GlobalNamespace::MenuTransitionsHelper,
    pub _lobbyGameStateModel: *mut crate::GlobalNamespace::LobbyGameStateModel,
    pub _lobbyPlayerPermissionsModel: *mut crate::GlobalNamespace::LobbyPlayerPermissionsModel,
    pub _beatmapCharacteristicCollection: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
    pub _beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub selectedLevelGameplaySetupDataChangedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::ILevelGameplaySetupData,
    >,
    pub gameStartedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::ILevelGameplaySetupData,
    >,
    pub gameStartCancelledEvent: *mut crate::System::Action,
    pub countdownStartedEvent: *mut crate::System::Action,
    pub countdownCancelledEvent: *mut crate::System::Action,
    pub songStillDownloadingEvent: *mut crate::System::Action,
    pub startTimeChangedEvent: *mut crate::System::Action,
    pub levelFinishedEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        *mut crate::GlobalNamespace::MultiplayerResultsData,
    >,
    pub levelDidGetDisconnectedEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::DisconnectedReason,
    >,
    pub lobbyDisconnectedEvent: *mut crate::System::Action,
    pub beforeSceneSwitchCallbackEvent: *mut crate::System::Action,
    pub lobbyStateChangedEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::MultiplayerLobbyState,
    >,
    pub startButtonEnabledEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::CannotStartGameReason,
    >,
    pub playerMissingEntitlementsChangedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
    >,
    pub _levelStartInitiated_k__BackingField: bool,
    pub _countdownStarted_k__BackingField: bool,
    pub _countdownEndTime_k__BackingField: i64,
    pub _isDisconnected_k__BackingField: bool,
    pub _disconnectedReason_k__BackingField: crate::GlobalNamespace::DisconnectedReason,
    pub _predictedStartTime: i64,
    pub _startTime: i64,
    pub _levelStartedOnTime: bool,
    pub _state: crate::GlobalNamespace::MultiplayerLobbyState,
    pub _cannotStartGameReason: crate::GlobalNamespace::CannotStartGameReason,
    pub _selectedLevelGameplaySetupData: *mut crate::GlobalNamespace::LevelGameplaySetupData,
}
#[cfg(feature = "LobbyGameStateController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LobbyGameStateController => ""
    ."LobbyGameStateController"
);
#[cfg(feature = "LobbyGameStateController")]
impl std::ops::Deref for crate::GlobalNamespace::LobbyGameStateController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl std::ops::DerefMut for crate::GlobalNamespace::LobbyGameStateController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl crate::GlobalNamespace::LobbyGameStateController {
    pub const kLongTimerMs: i64 = 60000i64;
    pub const kShortTimerMs: i64 = 5000i64;
    #[cfg(feature = "LobbyGameStateController+__c")]
    pub type __c = crate::GlobalNamespace::LobbyGameStateController___c;
    #[cfg(feature = "LobbyGameStateController+__c__DisplayClass98_0")]
    pub type __c__DisplayClass98_0 = crate::GlobalNamespace::LobbyGameStateController___c__DisplayClass98_0;
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Activate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDisconnectedState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDisconnectedState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", ())?;
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
    pub fn GetCurrentLevelIfGameStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCurrentLevelIfGameStarted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGameStateAndConfigurationAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("GetGameStateAndConfigurationAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerCancelCountdown(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerCancelCountdown", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerCancelledLevelStart(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerCancelledLevelStart", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerClearSelectedBeatmap(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerClearSelectedBeatmap", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerClearSelectedGameplayModifiers(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerClearSelectedGameplayModifiers", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerSetCountdownEndTime(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        countdownTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerSetCountdownEndTime", (userId, countdownTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerSetPlayersMissingEntitlementsToLevel(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playersMissingEntitlements: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerSetPlayersMissingEntitlementsToLevel",
                (userId, playersMissingEntitlements),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerSetSelectedBeatmap(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKeySerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerSetSelectedBeatmap",
                (userId, beatmapKeySerializable),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerSetSelectedGameplayModifiers(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        modifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerSetSelectedGameplayModifiers",
                (userId, modifiers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerSetStartGameTime(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerSetStartGameTime", (userId, startTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerStartedLevel(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKeyNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLevelDidDisconnect(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplayerLevelDidDisconnect", (disconnectedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLevelDidFinish(
        &mut self,
        multiplayerLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        >,
        multiplayerResultsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelDidFinish",
                (multiplayerLevelScenesTransitionSetupData, multiplayerResultsData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLevelLoaderCountdownFinished(
        &mut self,
        gameplaySetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILevelGameplaySetupData,
        >,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelLoaderCountdownFinished",
                (gameplaySetupData, beatmapLevelData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLevelLoaderStillDownloadingSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplayerLevelLoaderStillDownloadingSong", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerSessionManagerConnectionOwnerStateChanged(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerSessionManagerConnectionOwnerStateChanged",
                (connectedPlayer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerSessionManagerDisconnected(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerSessionManagerDisconnected",
                (disconnectedReason),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSetIsStartButtonEnabled(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cannotStartGameReason: crate::GlobalNamespace::CannotStartGameReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSetIsStartButtonEnabled", (userId, cannotStartGameReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleStartTimeChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleStartTimeChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCloseToStartGame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCloseToStartGame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PredictCountdownEndTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PredictCountdownEndTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartListeningToGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartListeningToGameStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartMultiplayerLevel(
        &mut self,
        gameplaySetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILevelGameplaySetupData,
        >,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartMultiplayerLevel",
                (gameplaySetupData, beatmapLevelData, beforeSceneSwitchCallback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StopListeningToGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopListeningToGameStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopLoading", ())?;
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
    pub fn add_beforeSceneSwitchCallbackEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beforeSceneSwitchCallbackEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_countdownCancelledEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_countdownCancelledEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_countdownStartedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_countdownStartedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_gameStartCancelledEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameStartCancelledEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_gameStartedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ILevelGameplaySetupData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameStartedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_levelDidGetDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelDidGetDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_levelFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::MultiplayerResultsData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_lobbyDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_lobbyDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_lobbyStateChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::MultiplayerLobbyState>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_lobbyStateChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_playerMissingEntitlementsChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerMissingEntitlementsChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_selectedLevelGameplaySetupDataChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ILevelGameplaySetupData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectedLevelGameplaySetupDataChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_songStillDownloadingEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_songStillDownloadingEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_startButtonEnabledEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::CannotStartGameReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startButtonEnabledEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_startTimeChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startTimeChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cannotStartGameReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::CannotStartGameReason> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::CannotStartGameReason = __cordl_object
            .invoke("get_cannotStartGameReason", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_countdownEndTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_countdownEndTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_countdownStarted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_countdownStarted", ())?;
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
    pub fn get_isDisconnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDisconnected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelStartInitiated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_levelStartInitiated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_predictedCountdownEndTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_predictedCountdownEndTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedLevelGameplaySetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILevelGameplaySetupData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILevelGameplaySetupData,
        > = __cordl_object.invoke("get_selectedLevelGameplaySetupData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_startTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_state(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerLobbyState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerLobbyState = __cordl_object
            .invoke("get_state", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_beforeSceneSwitchCallbackEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beforeSceneSwitchCallbackEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_countdownCancelledEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_countdownCancelledEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_countdownStartedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_countdownStartedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_gameStartCancelledEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameStartCancelledEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_gameStartedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ILevelGameplaySetupData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameStartedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelDidGetDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelDidGetDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::MultiplayerResultsData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_lobbyDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_lobbyDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_lobbyStateChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::MultiplayerLobbyState>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_lobbyStateChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerMissingEntitlementsChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerMissingEntitlementsChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_selectedLevelGameplaySetupDataChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ILevelGameplaySetupData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectedLevelGameplaySetupDataChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_songStillDownloadingEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_songStillDownloadingEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_startButtonEnabledEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::CannotStartGameReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startButtonEnabledEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_startTimeChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startTimeChangedEvent", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_disconnectedReason(
        &mut self,
        value: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disconnectedReason", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_state(
        &mut self,
        value: crate::GlobalNamespace::MultiplayerLobbyState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_state", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LobbyGameStateController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl AsRef<crate::GlobalNamespace::ILobbyGameStateController>
for crate::GlobalNamespace::LobbyGameStateController {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILobbyGameStateController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl AsMut<crate::GlobalNamespace::ILobbyGameStateController>
for crate::GlobalNamespace::LobbyGameStateController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILobbyGameStateController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl AsRef<crate::GlobalNamespace::ILobbyGameStateControllerBase>
for crate::GlobalNamespace::LobbyGameStateController {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILobbyGameStateControllerBase {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl AsMut<crate::GlobalNamespace::ILobbyGameStateControllerBase>
for crate::GlobalNamespace::LobbyGameStateController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILobbyGameStateControllerBase {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::LobbyGameStateController {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyGameStateController")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::LobbyGameStateController {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
