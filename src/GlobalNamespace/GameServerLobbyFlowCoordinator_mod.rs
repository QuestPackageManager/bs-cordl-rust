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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameServerLobbyFlowCoordinator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameServerLobbyFlowCoordinator";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BackButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BackButtonWasPressed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (topViewController))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DidActivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DidActivate", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (firstActivation, addedToHierarchy, screenSystemEnabling),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DidDeactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DidDeactivate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (removedFromHierarchy, screenSystemDisabling))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DismissViewControllersAndCoordinators(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("DismissViewControllersAndCoordinators")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DismissViewControllersAndCoordinators", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Finish(
        &mut self,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        withFadeOut: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Finish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Finish", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (finishedCallback, withFadeOut))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInitialGameState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("GetInitialGameState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInitialGameState", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLobbyType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameServerLobbyFlowCoordinatorBase_LobbyType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::GameServerLobbyFlowCoordinatorBase_LobbyType,
                0usize,
            >("GetLobbyType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLobbyType", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::GameServerLobbyFlowCoordinatorBase_LobbyType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalizedTitle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("GetLocalizedTitle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLocalizedTitle", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameBeforeSceneSwitchCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbyGameBeforeSceneSwitchCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameBeforeSceneSwitchCallback", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerCancelStartTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbyGameStateControllerCancelStartTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerCancelStartTime", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerCountdownCancelled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbyGameStateControllerCountdownCancelled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerCountdownCancelled", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerCountdownStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbyGameStateControllerCountdownStarted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerCountdownStarted", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerGameStarted(
        &mut self,
        levelGameplaySetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILevelGameplaySetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ILevelGameplaySetupData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleLobbyGameStateControllerGameStarted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerGameStarted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (levelGameplaySetupData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerGameStartedPresentView(
        &mut self,
        levelGameplaySetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILevelGameplaySetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ILevelGameplaySetupData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleLobbyGameStateControllerGameStartedPresentView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerGameStartedPresentView", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (levelGameplaySetupData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerGameStateChanged(
        &mut self,
        state: crate::GlobalNamespace::MultiplayerLobbyState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::MultiplayerLobbyState),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleLobbyGameStateControllerGameStateChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerGameStateChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerLevelDidGetDisconnected(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::DisconnectedReason),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleLobbyGameStateControllerLevelDidGetDisconnected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerLevelDidGetDisconnected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (disconnectedReason))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::MultiplayerResultsData,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleLobbyGameStateControllerLevelFinished")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerLevelFinished", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (multiplayerLevelScenesTransitionSetupData, multiplayerResultsData),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerLobbyDisconnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbyGameStateControllerLobbyDisconnected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerLobbyDisconnected", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerSongStillDownloading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbyGameStateControllerSongStillDownloading")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerSongStillDownloading", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateControllerStartTimeChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbyGameStateControllerStartTimeChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateControllerStartTimeChanged", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateStartButtonEnabled(
        &mut self,
        cannotStartGameReason: crate::GlobalNamespace::CannotStartGameReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::CannotStartGameReason),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleLobbyGameStateStartButtonEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyGameStateStartButtonEnabled", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cannotStartGameReason))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyPlayerPermissionsModelPermissionsChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbyPlayerPermissionsModelPermissionsChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyPlayerPermissionsModelPermissionsChanged", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyPlayersDataModelDidChange(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleLobbyPlayersDataModelDidChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbyPlayersDataModelDidChange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerCancelGameOrUnready(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbySetupViewControllerCancelGameOrUnready")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbySetupViewControllerCancelGameOrUnready", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerClearSelectedBeatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbySetupViewControllerClearSelectedBeatmap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbySetupViewControllerClearSelectedBeatmap", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerClearSelectedModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbySetupViewControllerClearSelectedModifiers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbySetupViewControllerClearSelectedModifiers", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerSelectBeatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbySetupViewControllerSelectBeatmap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbySetupViewControllerSelectBeatmap", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerSelectModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbySetupViewControllerSelectModifiers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbySetupViewControllerSelectModifiers", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbySetupViewControllerStartGameOrReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleLobbySetupViewControllerStartGameOrReady")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLobbySetupViewControllerStartGameOrReady", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerSetPlayersMissingEntitlementsToLevel(
        &mut self,
        playersMissingEntitlements: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleMenuRpcManagerSetPlayersMissingEntitlementsToLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleMenuRpcManagerSetPlayersMissingEntitlementsToLevel",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playersMissingEntitlements))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLevelSelectionFlowCoordinatorCancelSelectLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleMultiplayerLevelSelectionFlowCoordinatorCancelSelectLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "HandleMultiplayerLevelSelectionFlowCoordinatorCancelSelectLevel",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLevelSelectionFlowCoordinatorDidSelectLevel(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleMultiplayerLevelSelectionFlowCoordinatorDidSelectLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleMultiplayerLevelSelectionFlowCoordinatorDidSelectLevel",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerResultsViewControllerBackToLobbyPressed(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerResultsViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerResultsViewController,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleMultiplayerResultsViewControllerBackToLobbyPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleMultiplayerResultsViewControllerBackToLobbyPressed",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerResultsViewControllerBackToMenuPressed(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerResultsViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerResultsViewController,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleMultiplayerResultsViewControllerBackToMenuPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleMultiplayerResultsViewControllerBackToMenuPressed",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerSettingsPanelControllerPlayerActiveStateChanged(
        &mut self,
        isActive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleMultiplayerSettingsPanelControllerPlayerActiveStateChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "HandleMultiplayerSettingsPanelControllerPlayerActiveStateChanged",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (isActive))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleServerPlayerListViewControllerDidOpenInvitePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleServerPlayerListViewControllerDidOpenInvitePanel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleServerPlayerListViewControllerDidOpenInvitePanel",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleServerPlayerListViewControllerKickPlayer(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleServerPlayerListViewControllerKickPlayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleServerPlayerListViewControllerKickPlayer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleServerPlayerListViewControllerSelectSuggestedBeatmap(
        &mut self,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::BeatmapKey),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleServerPlayerListViewControllerSelectSuggestedBeatmap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleServerPlayerListViewControllerSelectSuggestedBeatmap",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beatmapKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleServerPlayerListViewControllerSelectSuggestedGameplayModifiers(
        &mut self,
        modifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleServerPlayerListViewControllerSelectSuggestedGameplayModifiers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "HandleServerPlayerListViewControllerSelectSuggestedGameplayModifiers",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (modifiers))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitialViewControllerWasPresented(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("InitialViewControllerWasPresented")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InitialViewControllerWasPresented", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("PresentBackButtonConfirmationDialog")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PresentBackButtonConfirmationDialog", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLobbyPlayerDataToViews(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetLobbyPlayerDataToViews")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetLobbyPlayerDataToViews", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerSelectedLevel(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionFlowCoordinator_State,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetLocalPlayerSelectedLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetLocalPlayerSelectedLevel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayersMissingLevelText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("SetPlayersMissingLevelText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetPlayersMissingLevelText", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTitle(
        &mut self,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTitle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetTitle", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (newViewController, animationType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupLobbyWithPermissions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("SetupLobbyWithPermissions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetupLobbyWithPermissions", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowBackButton(
        &mut self,
        show: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("ShowBackButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ShowBackButton", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (show))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowDisconnectDialogAndFinish(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::DisconnectedReason),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ShowDisconnectDialogAndFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ShowDisconnectDialogAndFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (disconnectedReason))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowSideViewControllers(
        &mut self,
        showSideViewControllers: bool,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, crate::HMUI::ViewController_AnimationType),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ShowSideViewControllers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ShowSideViewControllers", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (showSideViewControllers, animationType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TopViewControllerWillChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TopViewControllerWillChange", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (oldViewController, newViewController, animationType),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransitionDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("TransitionDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TransitionDidFinish", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransitionDidStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("TransitionDidStart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TransitionDidStart", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLocalPlayerIsActiveState(
        &mut self,
        isActive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UpdateLocalPlayerIsActiveState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateLocalPlayerIsActiveState", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (isActive))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _PresentBackButtonConfirmationDialog_b__70_0(
        &mut self,
        btnIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("<PresentBackButtonConfirmationDialog>b__70_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<PresentBackButtonConfirmationDialog>b__70_0", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (btnIndex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ShowDisconnectDialogAndFinish_b__107_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("<ShowDisconnectDialogAndFinish>b__107_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<ShowDisconnectDialogAndFinish>b__107_0", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didOpenInvitePanelEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didOpenInvitePanelEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didOpenInvitePanelEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didSetupEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didSetupEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didSetupEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_startGameOrReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_startGameOrReadyEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_startGameOrReadyEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_willFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_willFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_willFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isManaged(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isManaged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isManaged", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_isPartyOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isPartyOwner")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isPartyOwner", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_isPublicGame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isPublicGame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isPublicGame", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_isQuickPlayServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isQuickPlayServer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isQuickPlayServer", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_isQuickStartServer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isQuickStartServer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isQuickStartServer", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_rejoinQuickPlay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_rejoinQuickPlay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_rejoinQuickPlay", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didOpenInvitePanelEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didOpenInvitePanelEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didOpenInvitePanelEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSetupEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didSetupEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didSetupEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_startGameOrReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_startGameOrReadyEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_startGameOrReadyEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_willFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_willFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_willFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_rejoinQuickPlay(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_rejoinQuickPlay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_rejoinQuickPlay", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
