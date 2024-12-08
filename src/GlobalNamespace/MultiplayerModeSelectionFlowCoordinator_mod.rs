#[cfg(feature = "MultiplayerModeSelectionFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerModeSelectionFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _ambienceAudioClip: *mut crate::UnityEngine::AudioClip,
    pub _songPackMasksModel: *mut crate::GlobalNamespace::SongPackMasksModel,
    pub _gameServerBrowserFlowCoordinator: *mut crate::GlobalNamespace::GameServerBrowserFlowCoordinator,
    pub _gameServerLobbyFlowCoordinator: *mut crate::GlobalNamespace::GameServerLobbyFlowCoordinator,
    pub _multiplayerModeSelectionViewController: *mut crate::GlobalNamespace::MultiplayerModeSelectionViewController,
    pub _createServerViewController: *mut crate::GlobalNamespace::CreateServerViewController,
    pub _joinQuickPlayViewController: *mut crate::GlobalNamespace::JoinQuickPlayViewController,
    pub _serverCodeEntryViewController: *mut crate::GlobalNamespace::ServerCodeEntryViewController,
    pub _simpleDialogPromptViewController: *mut crate::GlobalNamespace::SimpleDialogPromptViewController,
    pub _joiningLobbyViewController: *mut crate::GlobalNamespace::JoiningLobbyViewController,
    pub _unifiedNetworkPlayerModel: *mut crate::GlobalNamespace::IUnifiedNetworkPlayerModel,
    pub _avatarSystemCollection: *mut crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _fadeInOutController: *mut crate::GlobalNamespace::FadeInOutController,
    pub _lobbyDataModelsManager: *mut crate::GlobalNamespace::LobbyDataModelsManager,
    pub _multiplayerLobbyConnectionController: *mut crate::GlobalNamespace::MultiplayerLobbyConnectionController,
    pub _multiplayerStatusModel: *mut crate::GlobalNamespace::IMultiplayerStatusModel,
    pub _quickPlaySetupModel: *mut crate::GlobalNamespace::IQuickPlaySetupModel,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _songPreviewPlayer: *mut crate::GlobalNamespace::SongPreviewPlayer,
    pub _analyticsModel: *mut crate::GlobalNamespace::IAnalyticsModel,
    pub _lobbyGameStateController: *mut crate::GlobalNamespace::ILobbyGameStateController,
    pub _xPlatformAuthFeatureFlag: *mut crate::GlobalNamespace::XPlatformAuthFeatureFlag,
    pub didFinishEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator,
    >,
    pub _joiningLobbyCancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _modeSelectionCancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _transitionFinishedTaskSource: *mut crate::System::Threading::Tasks::TaskCompletionSource_1<
        bool,
    >,
    pub _checkingAvailabilityTaskSource: *mut crate::System::Threading::Tasks::TaskCompletionSource_1<
        bool,
    >,
    pub _quickPlaySetupData: *mut crate::GlobalNamespace::QuickPlaySetupData,
    pub _lobbyDestination: *mut crate::GlobalNamespace::SelectMultiplayerLobbyDestination,
}
#[cfg(feature = "MultiplayerModeSelectionFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator => ""
    ."MultiplayerModeSelectionFlowCoordinator"
);
#[cfg(feature = "MultiplayerModeSelectionFlowCoordinator")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerModeSelectionFlowCoordinator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerModeSelectionFlowCoordinator")]
impl crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator {
    #[cfg(
        feature = "MultiplayerModeSelectionFlowCoordinator+_HandleConnectedPlayerManagerCreated_d__50"
    )]
    pub type _HandleConnectedPlayerManagerCreated_d__50 = crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator__HandleConnectedPlayerManagerCreated_d__50;
    #[cfg(
        feature = "MultiplayerModeSelectionFlowCoordinator+_ResolveAndPresentNextFlowCoordinator_d__52"
    )]
    pub type _ResolveAndPresentNextFlowCoordinator_d__52 = crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator__ResolveAndPresentNextFlowCoordinator_d__52;
    #[cfg(
        feature = "MultiplayerModeSelectionFlowCoordinator+_TryShowModeSelection_d__51"
    )]
    pub type _TryShowModeSelection_d__51 = crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator__TryShowModeSelection_d__51;
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
    pub fn HandleConnectedPlayerManagerCreated(
        &mut self,
        networkPlayerModel: *mut crate::GlobalNamespace::INetworkPlayerModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectedPlayerManagerCreated", (networkPlayerModel))?;
        Ok(__cordl_ret)
    }
    pub fn HandleCreateServerViewControllerDidFinish(
        &mut self,
        success: bool,
        data: crate::GlobalNamespace::CreateServerFormData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCreateServerViewControllerDidFinish", (success, data))?;
        Ok(__cordl_ret)
    }
    pub fn HandleGameServerBrowserFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: *mut crate::GlobalNamespace::GameServerBrowserFlowCoordinator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleGameServerBrowserFlowCoordinatorDidFinish",
                (flowCoordinator),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleGameServerLobbyFlowCoordinatorDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameServerLobbyFlowCoordinatorDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleGameServerLobbyFlowCoordinatorWillFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameServerLobbyFlowCoordinatorWillFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleJoinQuickPlayViewControllerDidFinish(
        &mut self,
        success: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleJoinQuickPlayViewControllerDidFinish", (success))?;
        Ok(__cordl_ret)
    }
    pub fn HandleJoiningLobbyViewControllerDidCancel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleJoiningLobbyViewControllerDidCancel", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLobbyConnectionControllerConnectionFailed(
        &mut self,
        connectionType: crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType,
        reason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLobbyConnectionControllerConnectionFailed",
                (connectionType, reason),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLobbyConnectionControllerConnectionSuccess(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplayerLobbyConnectionControllerConnectionSuccess", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLobbyConnectionControllerConnectionSuccessActivateModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLobbyConnectionControllerConnectionSuccessActivateModel",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLobbyControllerDidFinish(
        &mut self,
        viewController: *mut crate::GlobalNamespace::MultiplayerModeSelectionViewController,
        menuButton: crate::GlobalNamespace::MultiplayerModeSelectionViewController_MenuButton,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLobbyControllerDidFinish",
                (viewController, menuButton),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleServerCodeEntryViewControllerDidFinish(
        &mut self,
        success: bool,
        code: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleServerCodeEntryViewControllerDidFinish", (success, code))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PresentConnectionErrorDialog(
        &mut self,
        connectionType: crate::GlobalNamespace::MultiplayerLobbyConnectionController_LobbyConnectionType,
        reason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentConnectionErrorDialog", (connectionType, reason))?;
        Ok(__cordl_ret)
    }
    pub fn PresentMasterServerUnavailableErrorDialog(
        &mut self,
        reason: crate::GlobalNamespace::MultiplayerUnavailableReason,
        exception: *mut crate::System::Exception,
        maintenanceWindowEndTime: crate::System::Nullable_1<i64>,
        remoteLocalizedMessage: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PresentMasterServerUnavailableErrorDialog",
                (reason, exception, maintenanceWindowEndTime, remoteLocalizedMessage),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessDeeplinkingToLobby(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDeeplinkingToLobby", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolveAndPresentNextFlowCoordinator(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveAndPresentNextFlowCoordinator", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        lobbyDestination: *mut crate::GlobalNamespace::SelectMultiplayerLobbyDestination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (lobbyDestination))?;
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
    pub fn TryShowModeSelection(
        &mut self,
        shouldProvideInitialViewControllers: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TryShowModeSelection",
                (shouldProvideInitialViewControllers, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _PresentConnectionErrorDialog_b__53_0(
        &mut self,
        btnId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<PresentConnectionErrorDialog>b__53_0", (btnId))?;
        Ok(__cordl_ret)
    }
    pub fn _PresentConnectionErrorDialog_b__53_1(
        &mut self,
        btnId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<PresentConnectionErrorDialog>b__53_1", (btnId))?;
        Ok(__cordl_ret)
    }
    pub fn _PresentMasterServerUnavailableErrorDialog_b__54_0(
        &mut self,
        btnId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<PresentMasterServerUnavailableErrorDialog>b__54_0", (btnId))?;
        Ok(__cordl_ret)
    }
    pub fn _ResolveAndPresentNextFlowCoordinator_b__52_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<ResolveAndPresentNextFlowCoordinator>b__52_0", ())?;
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
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerModeSelectionFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
