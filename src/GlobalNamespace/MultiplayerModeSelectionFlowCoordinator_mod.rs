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
    pub fn HandleConnectedPlayerManagerCreated(
        &mut self,
        networkPlayerModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::INetworkPlayerModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectedPlayerManagerCreated", (networkPlayerModel))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameServerBrowserFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameServerBrowserFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleGameServerBrowserFlowCoordinatorDidFinish",
                (flowCoordinator),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameServerLobbyFlowCoordinatorDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameServerLobbyFlowCoordinatorDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameServerLobbyFlowCoordinatorWillFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameServerLobbyFlowCoordinatorWillFinish", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleJoiningLobbyViewControllerDidCancel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleJoiningLobbyViewControllerDidCancel", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLobbyConnectionControllerConnectionSuccess(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplayerLobbyConnectionControllerConnectionSuccess", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLobbyControllerDidFinish(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerModeSelectionViewController,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleServerCodeEntryViewControllerDidFinish(
        &mut self,
        success: bool,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleServerCodeEntryViewControllerDidFinish", (success, code))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn PresentMasterServerUnavailableErrorDialog(
        &mut self,
        reason: crate::GlobalNamespace::MultiplayerUnavailableReason,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        maintenanceWindowEndTime: crate::System::Nullable_1<i64>,
        remoteLocalizedMessage: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PresentMasterServerUnavailableErrorDialog",
                (reason, exception, maintenanceWindowEndTime, remoteLocalizedMessage),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessDeeplinkingToLobby(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDeeplinkingToLobby", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        lobbyDestination: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SelectMultiplayerLobbyDestination,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (lobbyDestination))?;
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ResolveAndPresentNextFlowCoordinator_b__52_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<ResolveAndPresentNextFlowCoordinator>b__52_0", ())?;
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
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
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
