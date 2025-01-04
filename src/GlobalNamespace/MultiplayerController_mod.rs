#[cfg(feature = "MultiplayerController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _loadingEnvironment: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _multiplayerLevelSceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
    >,
    pub _gameScenesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameScenesManager,
    >,
    pub _playersManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerPlayersManager,
    >,
    pub _sceneStartSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SceneStartSyncController,
    >,
    pub _songStartSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongStartSyncController,
    >,
    pub _multiplayerLevelFinishedController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLevelFinishedController,
    >,
    pub _fadeInOutController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FadeInOutController,
    >,
    pub _multiplayerSessionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
    pub _introAnimationController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerIntroAnimationController,
    >,
    pub _outroAnimationController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerOutroAnimationController,
    >,
    pub _menuRpcManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMenuRpcManager,
    >,
    pub _gameplayRpcManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IGameplayRpcManager,
    >,
    pub _sceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayCoreSceneSetupData,
    >,
    pub _diContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _badgesProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerBadgesProvider,
    >,
    pub stateChangedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::GlobalNamespace::MultiplayerController_State>,
    >,
    pub _startTime: f32,
    pub _localPlayerSyncStartState: crate::GlobalNamespace::MultiplayerPlayerStartState,
    pub _state: crate::GlobalNamespace::MultiplayerController_State,
    pub _sessionGameId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _resultsData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerResultsData,
    >,
    pub _playersSpecificSettingsAtGameStartModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
    >,
    pub _timeoutGetGameStateCoroutine: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Coroutine,
    >,
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
    pub fn ChangeState(
        &mut self,
        newState: crate::GlobalNamespace::MultiplayerController_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeState", (newState))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAndBindPlayersSpecificSettingsAtGameStartModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateAndBindPlayersSpecificSettingsAtGameStartModel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EndGameplay(
        &mut self,
        localPlayerResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
        otherPlayerResults: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndGameplay", (localPlayerResults, otherPlayerResults))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleAllResultsCollected(
        &mut self,
        localPlayerResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
        otherPlayerResults: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleDidSwitchPlayerToInactive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidSwitchPlayerToInactive", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleInitialGetGameStateFailed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInitialGetGameStateFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleIsDisconnectedDuringLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleIsDisconnectedDuringLoading", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleOutroAnimationDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOutroAnimationDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRpcReturnToMenu(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRpcReturnToMenu", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSceneStartSyncControllerSyncStartDidFail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSceneStartSyncControllerSyncStartDidFail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSceneStartSyncControllerSyncStartDidReceiveTooLate(
        &mut self,
        sessionGameId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSceneStartSyncControllerSyncStartDidReceiveTooLate",
                (sessionGameId),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSceneStartSyncControllerSyncStartDidSuccess(
        &mut self,
        sessionGameId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSceneStartSyncControllerSyncStartDidSuccess",
                (sessionGameId),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSetMultiplayerGameState(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameState: crate::GlobalNamespace::MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSetMultiplayerGameState", (userId, gameState))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleSongStartSyncControllerSyncStartFailed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSongStartSyncControllerSyncStartFailed", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PerformSongStartSync(
        &mut self,
        localPlayerSyncState: crate::GlobalNamespace::MultiplayerPlayerStartState,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("PerformSongStartSync", (localPlayerSyncState))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn StartSceneLoadSync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSceneLoadSync", ())?;
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
    pub fn add_stateChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::MultiplayerController_State>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_stateChangedEvent", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn remove_stateChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::MultiplayerController_State>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_stateChangedEvent", (value))?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerController_State {
    #[default]
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
