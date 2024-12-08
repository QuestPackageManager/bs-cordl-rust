#[cfg(feature = "MultiplayerController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _loadingEnvironment: *mut crate::UnityEngine::GameObject,
    pub _multiplayerLevelSceneSetupData: *mut MultiplayerLevelScenesTransitionSetupDataSO,
    pub _gameScenesManager: *mut GameScenesManager,
    pub _playersManager: *mut MultiplayerPlayersManager,
    pub _sceneStartSyncController: *mut SceneStartSyncController,
    pub _songStartSyncController: *mut SongStartSyncController,
    pub _multiplayerLevelFinishedController: *mut MultiplayerLevelFinishedController,
    pub _fadeInOutController: *mut FadeInOutController,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _introAnimationController: *mut MultiplayerIntroAnimationController,
    pub _outroAnimationController: *mut MultiplayerOutroAnimationController,
    pub _menuRpcManager: *mut IMenuRpcManager,
    pub _gameplayRpcManager: *mut IGameplayRpcManager,
    pub _sceneSetupData: *mut GameplayCoreSceneSetupData,
    pub _diContainer: *mut crate::Zenject::DiContainer,
    pub _badgesProvider: *mut MultiplayerBadgesProvider,
    pub stateChangedEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::MultiplayerController_State,
    >,
    pub _startTime: f32,
    pub _localPlayerSyncStartState: MultiplayerPlayerStartState,
    pub _state: crate::GlobalNamespace::MultiplayerController_State,
    pub _sessionGameId: *mut crate::System::String,
    pub _resultsData: *mut MultiplayerResultsData,
    pub _playersSpecificSettingsAtGameStartModel: *mut PlayersSpecificSettingsAtGameStartModel,
    pub _timeoutGetGameStateCoroutine: *mut crate::UnityEngine::Coroutine,
}
#[cfg(feature = "MultiplayerController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerController => ""."MultiplayerController"
);
#[cfg(feature = "MultiplayerController")]
impl std::ops::Deref for MultiplayerController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerController")]
impl std::ops::DerefMut for MultiplayerController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerController")]
impl MultiplayerController {
    pub const kDisconnectWaitForInitTimeSeconds: f32 = 0.5f32;
    pub const kGetMultiplayerGameStateTimeoutSeconds: f32 = 20f32;
    pub const kMinAnimationDurationPercentage: f32 = 0.75f32;
    pub const kSongTimeToSongStartSyncTimeOffsetMs: i64 = -600i64;
    #[cfg(feature = "MultiplayerController+__c")]
    pub type __c = crate::GlobalNamespace::MultiplayerController___c;
    #[cfg(feature = "MultiplayerController+_PerformSongStartSync_d__39")]
    pub type _PerformSongStartSync_d__39 = crate::GlobalNamespace::MultiplayerController__PerformSongStartSync_d__39;
    #[cfg(feature = "MultiplayerController+__c__DisplayClass45_0")]
    pub type __c__DisplayClass45_0 = crate::GlobalNamespace::MultiplayerController___c__DisplayClass45_0;
    #[cfg(feature = "MultiplayerController+State")]
    pub type State = crate::GlobalNamespace::MultiplayerController_State;
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
    pub fn HandleSetMultiplayerGameState(
        &mut self,
        userId: *mut crate::System::String,
        gameState: MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSetMultiplayerGameState", (userId, gameState))?;
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
    pub fn EndGameplay(
        &mut self,
        localPlayerResults: *mut MultiplayerLevelCompletionResults,
        otherPlayerResults: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndGameplay", (localPlayerResults, otherPlayerResults))?;
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
    pub fn PerformSongStartSync(
        &mut self,
        localPlayerSyncState: MultiplayerPlayerStartState,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("PerformSongStartSync", (localPlayerSyncState))?;
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
    pub fn HandleAllResultsCollected(
        &mut self,
        localPlayerResults: *mut MultiplayerLevelCompletionResults,
        otherPlayerResults: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut MultiplayerLevelCompletionResults,
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
    pub fn HandleDisconnected(
        &mut self,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDisconnected", (disconnectedReason))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerController")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerController {
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
