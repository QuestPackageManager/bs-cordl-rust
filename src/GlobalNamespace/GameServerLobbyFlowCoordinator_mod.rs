#[cfg(feature = "GameServerLobbyFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerLobbyFlowCoordinator {
    __cordl_parent: crate::GlobalNamespace::GameServerLobbyFlowCoordinatorBase,
    pub _screenMode: quest_hook::libil2cpp::Gc<crate::HMUI::ScreenModeSO>,
    pub _ambienceAudioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub _serverPlayerListViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ServerPlayerListViewController,
    >,
    pub _selectModifiersViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SelectModifiersViewController,
    >,
    pub _multiplayerLevelSelectionFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLevelSelectionFlowCoordinator,
    >,
    pub _multiplayerResultsViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerResultsViewController,
    >,
    pub _simpleDialogPromptViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SimpleDialogPromptViewController,
    >,
    pub _connectionErrorDialogViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ConnectionErrorDialogViewController,
    >,
    pub _multiplayerSettingsPanelController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerSettingsPanelController,
    >,
    pub _gameplaySetupViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplaySetupViewController,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub _multiplayerLobbyController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLobbyController,
    >,
    pub _fadeInOutController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FadeInOutController,
    >,
    pub _centerStageScreenController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CenterStageScreenController,
    >,
    pub _lobbyStateDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ILobbyStateDataModel,
    >,
    pub _lobbyGameStateModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LobbyGameStateModel,
    >,
    pub _lobbyPlayersDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ILobbyPlayersDataModel,
    >,
    pub _lobbyGameStateController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ILobbyGameStateController,
    >,
    pub _lobbyPlayerPermissionsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LobbyPlayerPermissionsModel,
    >,
    pub _lobbySetupViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LobbySetupViewController,
    >,
    pub _unifiedNetworkPlayerModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IUnifiedNetworkPlayerModel,
    >,
    pub _screenModeController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ScreenModeController,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _platformLeaderboardsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlatformLeaderboardsModel,
    >,
    pub _songPreviewPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongPreviewPlayer,
    >,
    pub _analyticsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAnalyticsModel,
    >,
    pub _lastSimpleLevelSelectionFlowCoordinatorState: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
    >,
    pub _isAlreadyFinishing: bool,
    pub _missingEntitlementsStringBuilder: quest_hook::libil2cpp::Gc<
        crate::System::Text::StringBuilder,
    >,
    pub _playerIdsWithoutEntitlements: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _canStartGameCts: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
    pub willFinishEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub startGameOrReadyEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub didSetupEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub didOpenInvitePanelEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _rejoinQuickPlay_k__BackingField: bool,
}
#[cfg(feature = "GameServerLobbyFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServerLobbyFlowCoordinator
    => ""."GameServerLobbyFlowCoordinator"
);
#[cfg(feature = "GameServerLobbyFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerLobbyFlowCoordinator {
    type Target = crate::GlobalNamespace::GameServerLobbyFlowCoordinatorBase;
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
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackButtonWasPressed", (topViewController))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn DismissViewControllersAndCoordinators(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DismissViewControllersAndCoordinators", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Finish(
        &mut self,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        withFadeOut: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", (finishedCallback, withFadeOut))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInitialGameState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetInitialGameState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLobbyType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameServerLobbyFlowCoordinatorBase_LobbyType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::GameServerLobbyFlowCoordinatorBase_LobbyType = __cordl_object
            .invoke("GetLobbyType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalizedTitle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetLocalizedTitle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameBeforeSceneSwitchCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameBeforeSceneSwitchCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerCancelStartTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerCancelStartTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerCountdownCancelled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerCountdownCancelled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerCountdownStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerCountdownStarted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerGameStarted(
        &mut self,
        levelGameplaySetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILevelGameplaySetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLobbyGameStateControllerGameStarted",
                (levelGameplaySetupData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerGameStartedPresentView(
        &mut self,
        levelGameplaySetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILevelGameplaySetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLobbyGameStateControllerGameStartedPresentView",
                (levelGameplaySetupData),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerLevelFinished(
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
                "HandleLobbyGameStateControllerLevelFinished",
                (multiplayerLevelScenesTransitionSetupData, multiplayerResultsData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerLobbyDisconnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerLobbyDisconnected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerSongStillDownloading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerSongStillDownloading", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerStartTimeChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerStartTimeChanged", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyPlayerPermissionsModelPermissionsChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyPlayerPermissionsModelPermissionsChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyPlayersDataModelDidChange(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyPlayersDataModelDidChange", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerCancelGameOrUnready(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerCancelGameOrUnready", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerClearSelectedBeatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerClearSelectedBeatmap", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerClearSelectedModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerClearSelectedModifiers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerSelectBeatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerSelectBeatmap", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerSelectModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerSelectModifiers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerStartGameOrReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbySetupViewControllerStartGameOrReady", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerSetPlayersMissingEntitlementsToLevel(
        &mut self,
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
                (playersMissingEntitlements),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLevelSelectionFlowCoordinatorDidSelectLevel(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelSelectionFlowCoordinatorDidSelectLevel",
                (state),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerResultsViewControllerBackToLobbyPressed(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerResultsViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerResultsViewControllerBackToLobbyPressed",
                (viewController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerResultsViewControllerBackToMenuPressed(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerResultsViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerResultsViewControllerBackToMenuPressed",
                (viewController),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleServerPlayerListViewControllerDidOpenInvitePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleServerPlayerListViewControllerDidOpenInvitePanel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleServerPlayerListViewControllerKickPlayer(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleServerPlayerListViewControllerKickPlayer", (userId))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleServerPlayerListViewControllerSelectSuggestedGameplayModifiers(
        &mut self,
        modifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleServerPlayerListViewControllerSelectSuggestedGameplayModifiers",
                (modifiers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitialViewControllerWasPresented(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitialViewControllerWasPresented", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PresentBackButtonConfirmationDialog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentBackButtonConfirmationDialog", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLobbyPlayerDataToViews(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLobbyPlayerDataToViews", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerSelectedLevel(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerSelectedLevel", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayersMissingLevelText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayersMissingLevelText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTitle(
        &mut self,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTitle", (newViewController, animationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupLobbyWithPermissions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupLobbyWithPermissions", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
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
        Ok(__cordl_ret.into())
    }
    pub fn TransitionDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TransitionDidStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionDidStart", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ShowDisconnectDialogAndFinish_b__107_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<ShowDisconnectDialogAndFinish>b__107_0", ())?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didOpenInvitePanelEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didOpenInvitePanelEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didSetupEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSetupEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_startGameOrReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startGameOrReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_willFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_willFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isManaged(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isManaged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPartyOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPartyOwner", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPublicGame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPublicGame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isQuickPlayServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isQuickPlayServer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isQuickStartServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isQuickStartServer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rejoinQuickPlay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_rejoinQuickPlay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didOpenInvitePanelEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didOpenInvitePanelEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSetupEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSetupEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_startGameOrReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startGameOrReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_willFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_willFinishEvent", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
