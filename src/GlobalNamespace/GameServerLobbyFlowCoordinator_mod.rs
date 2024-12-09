#[cfg(feature = "GameServerLobbyFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerLobbyFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _screenMode: *mut crate::HMUI::ScreenModeSO,
    pub _ambienceAudioClip: *mut crate::UnityEngine::AudioClip,
    pub _serverPlayerListViewController: *mut crate::GlobalNamespace::ServerPlayerListViewController,
    pub _selectModifiersViewController: *mut crate::GlobalNamespace::SelectModifiersViewController,
    pub _multiplayerLevelSelectionFlowCoordinator: *mut crate::GlobalNamespace::MultiplayerLevelSelectionFlowCoordinator,
    pub _multiplayerResultsViewController: *mut crate::GlobalNamespace::MultiplayerResultsViewController,
    pub _simpleDialogPromptViewController: *mut crate::GlobalNamespace::SimpleDialogPromptViewController,
    pub _connectionErrorDialogViewController: *mut crate::GlobalNamespace::ConnectionErrorDialogViewController,
    pub _multiplayerSettingsPanelController: *mut crate::GlobalNamespace::MultiplayerSettingsPanelController,
    pub _gameplaySetupViewController: *mut crate::GlobalNamespace::GameplaySetupViewController,
    pub _multiplayerLobbyController: *mut crate::GlobalNamespace::MultiplayerLobbyController,
    pub _fadeInOutController: *mut crate::GlobalNamespace::FadeInOutController,
    pub _centerStageScreenController: *mut crate::GlobalNamespace::CenterStageScreenController,
    pub _lobbyStateDataModel: *mut crate::GlobalNamespace::ILobbyStateDataModel,
    pub _lobbyGameStateModel: *mut crate::GlobalNamespace::LobbyGameStateModel,
    pub _lobbyPlayersDataModel: *mut crate::GlobalNamespace::ILobbyPlayersDataModel,
    pub _lobbyGameStateController: *mut crate::GlobalNamespace::ILobbyGameStateController,
    pub _lobbyPlayerPermissionsModel: *mut crate::GlobalNamespace::LobbyPlayerPermissionsModel,
    pub _lobbySetupViewController: *mut crate::GlobalNamespace::LobbySetupViewController,
    pub _unifiedNetworkPlayerModel: *mut crate::GlobalNamespace::IUnifiedNetworkPlayerModel,
    pub _screenModeController: *mut crate::HMUI::ScreenModeController,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _platformLeaderboardsModel: *mut crate::GlobalNamespace::PlatformLeaderboardsModel,
    pub _songPreviewPlayer: *mut crate::GlobalNamespace::SongPreviewPlayer,
    pub _analyticsModel: *mut crate::GlobalNamespace::IAnalyticsModel,
    pub _lastSimpleLevelSelectionFlowCoordinatorState: *mut crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
    pub _isAlreadyFinishing: bool,
    pub _missingEntitlementsStringBuilder: *mut crate::System::Text::StringBuilder,
    pub _playerIdsWithoutEntitlements: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _canStartGameCts: *mut crate::System::Threading::CancellationTokenSource,
    pub willFinishEvent: *mut crate::System::Action,
    pub didFinishEvent: *mut crate::System::Action,
    pub startGameOrReadyEvent: *mut crate::System::Action,
    pub didSetupEvent: *mut crate::System::Action,
    pub didOpenInvitePanelEvent: *mut crate::System::Action,
    pub _rejoinQuickPlay_k__BackingField: bool,
}
#[cfg(feature = "GameServerLobbyFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServerLobbyFlowCoordinator
    => ""."GameServerLobbyFlowCoordinator"
);
#[cfg(feature = "GameServerLobbyFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerLobbyFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerLobbyFlowCoordinator")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServerLobbyFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerLobbyFlowCoordinator")]
impl crate::GlobalNamespace::GameServerLobbyFlowCoordinator {
    pub const kMaxPredictedStartTimeDifferenceMs: i64 = 1500i64;
    pub const kPlayersMissingEntitlementKey: &'static str = "LABEL_PLAYERS_MISSING_ENTITLEMENT";
    #[cfg(feature = "GameServerLobbyFlowCoordinator+LobbyType")]
    pub type LobbyType = crate::GlobalNamespace::GameServerLobbyFlowCoordinator_LobbyType;
    #[cfg(feature = "GameServerLobbyFlowCoordinator+__c__DisplayClass71_0")]
    pub type __c__DisplayClass71_0 = crate::GlobalNamespace::GameServerLobbyFlowCoordinator___c__DisplayClass71_0;
    #[cfg(feature = "GameServerLobbyFlowCoordinator+__c__DisplayClass86_0")]
    pub type __c__DisplayClass86_0 = crate::GlobalNamespace::GameServerLobbyFlowCoordinator___c__DisplayClass86_0;
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackButtonWasPressed", (topViewController))?;
        Ok(__cordl_ret)
    }
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret)
    }
    pub fn DismissViewControllersAndCoordinators(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DismissViewControllersAndCoordinators", ())?;
        Ok(__cordl_ret)
    }
    pub fn Finish(
        &mut self,
        finishedCallback: *mut crate::System::Action,
        withFadeOut: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", (finishedCallback, withFadeOut))?;
        Ok(__cordl_ret)
    }
    pub fn GetInitialGameState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetInitialGameState", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLobbyType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameServerLobbyFlowCoordinator_LobbyType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::GameServerLobbyFlowCoordinator_LobbyType = __cordl_object
            .invoke("GetLobbyType", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLocalizedTitle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetLocalizedTitle", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameBeforeSceneSwitchCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameBeforeSceneSwitchCallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerCancelStartTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerCancelStartTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerCountdownCancelled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerCountdownCancelled", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerCountdownStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerCountdownStarted", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerGameStarted(
        &mut self,
        levelGameplaySetupData: *mut crate::GlobalNamespace::ILevelGameplaySetupData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLobbyGameStateControllerGameStarted",
                (levelGameplaySetupData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerGameStartedPresentView(
        &mut self,
        levelGameplaySetupData: *mut crate::GlobalNamespace::ILevelGameplaySetupData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLobbyGameStateControllerGameStartedPresentView",
                (levelGameplaySetupData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerGameStateChanged(
        &mut self,
        state: crate::GlobalNamespace::MultiplayerLobbyState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerGameStateChanged", (state))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerLevelDidGetDisconnected(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLobbyGameStateControllerLevelDidGetDisconnected",
                (disconnectedReason),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerLevelFinished(
        &mut self,
        multiplayerLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        multiplayerResultsData: *mut crate::GlobalNamespace::MultiplayerResultsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLobbyGameStateControllerLevelFinished",
                (multiplayerLevelScenesTransitionSetupData, multiplayerResultsData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerLobbyDisconnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerLobbyDisconnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerSongStillDownloading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerSongStillDownloading", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerStartTimeChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerStartTimeChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateStartButtonEnabled(
        &mut self,
        cannotStartGameReason: crate::GlobalNamespace::CannotStartGameReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateStartButtonEnabled", (cannotStartGameReason))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyPlayerPermissionsModelPermissionsChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyPlayerPermissionsModelPermissionsChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyPlayersDataModelDidChange(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyPlayersDataModelDidChange", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbySetupViewControllerCancelGameOrUnready(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerCancelGameOrUnready", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbySetupViewControllerClearSelectedBeatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerClearSelectedBeatmap", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbySetupViewControllerClearSelectedModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerClearSelectedModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbySetupViewControllerSelectBeatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerSelectBeatmap", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbySetupViewControllerSelectModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerSelectModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbySetupViewControllerStartGameOrReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerStartGameOrReady", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerSetPlayersMissingEntitlementsToLevel(
        &mut self,
        playersMissingEntitlements: *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerSetPlayersMissingEntitlementsToLevel",
                (playersMissingEntitlements),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLevelSelectionFlowCoordinatorCancelSelectLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelSelectionFlowCoordinatorCancelSelectLevel",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLevelSelectionFlowCoordinatorDidSelectLevel(
        &mut self,
        state: *mut crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelSelectionFlowCoordinatorDidSelectLevel",
                (state),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerResultsViewControllerBackToLobbyPressed(
        &mut self,
        viewController: *mut crate::GlobalNamespace::MultiplayerResultsViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerResultsViewControllerBackToLobbyPressed",
                (viewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerResultsViewControllerBackToMenuPressed(
        &mut self,
        viewController: *mut crate::GlobalNamespace::MultiplayerResultsViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerResultsViewControllerBackToMenuPressed",
                (viewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerSettingsPanelControllerPlayerActiveStateChanged(
        &mut self,
        isActive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerSettingsPanelControllerPlayerActiveStateChanged",
                (isActive),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleServerPlayerListViewControllerDidOpenInvitePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleServerPlayerListViewControllerDidOpenInvitePanel", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleServerPlayerListViewControllerKickPlayer(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleServerPlayerListViewControllerKickPlayer", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn HandleServerPlayerListViewControllerSelectSuggestedBeatmap(
        &mut self,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleServerPlayerListViewControllerSelectSuggestedBeatmap",
                (beatmapKey),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleServerPlayerListViewControllerSelectSuggestedGameplayModifiers(
        &mut self,
        modifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleServerPlayerListViewControllerSelectSuggestedGameplayModifiers",
                (modifiers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InitialViewControllerWasPresented(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitialViewControllerWasPresented", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PresentBackButtonConfirmationDialog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentBackButtonConfirmationDialog", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLobbyPlayerDataToViews(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLobbyPlayerDataToViews", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn SetPlayersMissingLevelText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayersMissingLevelText", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetTitle(
        &mut self,
        newViewController: *mut crate::HMUI::ViewController,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTitle", (newViewController, animationType))?;
        Ok(__cordl_ret)
    }
    pub fn SetupLobbyWithPermissions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupLobbyWithPermissions", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShowBackButton(
        &mut self,
        show: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowBackButton", (show))?;
        Ok(__cordl_ret)
    }
    pub fn ShowDisconnectDialogAndFinish(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowDisconnectDialogAndFinish", (disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn ShowSideViewControllers(
        &mut self,
        showSideViewControllers: bool,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ShowSideViewControllers",
                (showSideViewControllers, animationType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: *mut crate::HMUI::ViewController,
        newViewController: *mut crate::HMUI::ViewController,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TopViewControllerWillChange",
                (oldViewController, newViewController, animationType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TransitionDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn TransitionDidStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionDidStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateLocalPlayerIsActiveState(
        &mut self,
        isActive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLocalPlayerIsActiveState", (isActive))?;
        Ok(__cordl_ret)
    }
    pub fn _PresentBackButtonConfirmationDialog_b__70_0(
        &mut self,
        btnIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<PresentBackButtonConfirmationDialog>b__70_0", (btnIndex))?;
        Ok(__cordl_ret)
    }
    pub fn _ShowDisconnectDialogAndFinish_b__106_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<ShowDisconnectDialogAndFinish>b__106_0", ())?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didOpenInvitePanelEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didOpenInvitePanelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didSetupEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSetupEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_startGameOrReadyEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startGameOrReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_willFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_willFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isManaged(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isManaged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isPartyOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPartyOwner", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isPublicGame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPublicGame", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isQuickPlayServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isQuickPlayServer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isQuickStartServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isQuickStartServer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rejoinQuickPlay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_rejoinQuickPlay", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didOpenInvitePanelEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didOpenInvitePanelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSetupEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSetupEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_startGameOrReadyEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startGameOrReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_willFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_willFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rejoinQuickPlay(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rejoinQuickPlay", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameServerLobbyFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameServerLobbyFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameServerLobbyFlowCoordinator+LobbyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameServerLobbyFlowCoordinator_LobbyType {
    ClientSetup = 1i32,
    HostSetup = 0i32,
    Party = 3i32,
    QuickPlayLobby = 2i32,
}
#[cfg(feature = "GameServerLobbyFlowCoordinator+LobbyType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameServerLobbyFlowCoordinator_LobbyType => ""
    ."GameServerLobbyFlowCoordinator/LobbyType"
);
