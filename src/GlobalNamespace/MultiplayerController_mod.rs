#[cfg(feature = "MultiplayerController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _loadingEnvironment: *mut crate::UnityEngine::GameObject,
    pub _multiplayerLevelSceneSetupData: *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
    pub _gameScenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    pub _playersManager: *mut crate::GlobalNamespace::MultiplayerPlayersManager,
    pub _sceneStartSyncController: *mut crate::GlobalNamespace::SceneStartSyncController,
    pub _songStartSyncController: *mut crate::GlobalNamespace::SongStartSyncController,
    pub _multiplayerLevelFinishedController: *mut crate::GlobalNamespace::MultiplayerLevelFinishedController,
    pub _fadeInOutController: *mut crate::GlobalNamespace::FadeInOutController,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _introAnimationController: *mut crate::GlobalNamespace::MultiplayerIntroAnimationController,
    pub _outroAnimationController: *mut crate::GlobalNamespace::MultiplayerOutroAnimationController,
    pub _menuRpcManager: *mut crate::GlobalNamespace::IMenuRpcManager,
    pub _gameplayRpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
    pub _sceneSetupData: *mut crate::GlobalNamespace::GameplayCoreSceneSetupData,
    pub _diContainer: *mut crate::Zenject::DiContainer,
    pub _badgesProvider: *mut crate::GlobalNamespace::MultiplayerBadgesProvider,
    pub stateChangedEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::MultiplayerController_State,
    >,
    pub _startTime: f32,
    pub _localPlayerSyncStartState: crate::GlobalNamespace::MultiplayerPlayerStartState,
    pub _state: crate::GlobalNamespace::MultiplayerController_State,
    pub _sessionGameId: *mut crate::System::String,
    pub _resultsData: *mut crate::GlobalNamespace::MultiplayerResultsData,
    pub _playersSpecificSettingsAtGameStartModel: *mut crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
    pub _timeoutGetGameStateCoroutine: *mut crate::UnityEngine::Coroutine,
}
#[cfg(feature = "MultiplayerController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerController => ""
    ."MultiplayerController"
);
#[cfg(feature = "MultiplayerController")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerController")]
impl crate::GlobalNamespace::MultiplayerController {
    pub const kDisconnectWaitForInitTimeSeconds: f32 = 0.5f32;
    pub const kGetMultiplayerGameStateTimeoutSeconds: f32 = 20f32;
    pub const kMinAnimationDurationPercentage: f32 = 0.75f32;
    pub const kSongTimeToSongStartSyncTimeOffsetMs: i64 = -600i64;
    #[cfg(feature = "MultiplayerController+State")]
    pub type State = crate::GlobalNamespace::MultiplayerController_State;
    #[cfg(feature = "MultiplayerController+_PerformSongStartSync_d__39")]
    pub type _PerformSongStartSync_d__39 = crate::GlobalNamespace::MultiplayerController__PerformSongStartSync_d__39;
    #[cfg(feature = "MultiplayerController+__c")]
    pub type __c = crate::GlobalNamespace::MultiplayerController___c;
    #[cfg(feature = "MultiplayerController+__c__DisplayClass45_0")]
    pub type __c__DisplayClass45_0 = crate::GlobalNamespace::MultiplayerController___c__DisplayClass45_0;
    pub fn ChangeState(
        &mut self,
        newState: crate::GlobalNamespace::MultiplayerController_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeState", (newState))?;
        Ok(__cordl_ret)
    }
    pub fn CreateAndBindPlayersSpecificSettingsAtGameStartModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateAndBindPlayersSpecificSettingsAtGameStartModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn EndGameplay(
        &mut self,
        localPlayerResults: *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        otherPlayerResults: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndGameplay", (localPlayerResults, otherPlayerResults))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentSongTime(
        &mut self,
        songStartSyncTime: i64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetCurrentSongTime", (songStartSyncTime))?;
        Ok(__cordl_ret)
    }
    pub fn GetSongStartSyncTime(
        &mut self,
        introAnimationStartSyncTime: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("GetSongStartSyncTime", (introAnimationStartSyncTime))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAllResultsCollected(
        &mut self,
        localPlayerResults: *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        otherPlayerResults: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleAllResultsCollected",
                (localPlayerResults, otherPlayerResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleDidSwitchPlayerToInactive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidSwitchPlayerToInactive", ())?;
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
    pub fn HandleInitialGetGameStateFailed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInitialGetGameStateFailed", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleIsDisconnectedDuringLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleIsDisconnectedDuringLoading", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleOutroAnimationDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOutroAnimationDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleRpcReturnToMenu(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRpcReturnToMenu", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSceneStartSyncControllerSyncStartDidFail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSceneStartSyncControllerSyncStartDidFail", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSceneStartSyncControllerSyncStartDidReceiveTooLate(
        &mut self,
        sessionGameId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSceneStartSyncControllerSyncStartDidReceiveTooLate",
                (sessionGameId),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSceneStartSyncControllerSyncStartDidSuccess(
        &mut self,
        sessionGameId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSceneStartSyncControllerSyncStartDidSuccess",
                (sessionGameId),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSetMultiplayerGameState(
        &mut self,
        userId: *mut crate::System::String,
        gameState: crate::GlobalNamespace::MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSetMultiplayerGameState", (userId, gameState))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSongStartSyncControllerSyncResume(
        &mut self,
        introAnimationStartSyncTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSongStartSyncControllerSyncResume",
                (introAnimationStartSyncTime),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSongStartSyncControllerSyncStartFailed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSongStartSyncControllerSyncStartFailed", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSongStartSyncControllerSyncStartSuccess(
        &mut self,
        introAnimationStartSyncTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSongStartSyncControllerSyncStartSuccess",
                (introAnimationStartSyncTime),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn PerformSongStartSync(
        &mut self,
        localPlayerSyncState: crate::GlobalNamespace::MultiplayerPlayerStartState,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("PerformSongStartSync", (localPlayerSyncState))?;
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
    pub fn StartGameplay(
        &mut self,
        introAnimationStartSyncTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartGameplay", (introAnimationStartSyncTime))?;
        Ok(__cordl_ret)
    }
    pub fn StartSceneLoadSync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSceneLoadSync", ())?;
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
    pub fn add_stateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::MultiplayerController_State,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_stateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_state(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerController_State,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerController_State = __cordl_object
            .invoke("get_state", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_stateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::MultiplayerController_State,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_stateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerController+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerController_State {
    CheckingLobbyState = 0i32,
    Finished = 6i32,
    Gameplay = 4i32,
    Intro = 3i32,
    Outro = 5i32,
    SongStartSync = 2i32,
    WaitingForPlayers = 1i32,
}
#[cfg(feature = "MultiplayerController+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerController_State =>
    ""."MultiplayerController/State"
);
