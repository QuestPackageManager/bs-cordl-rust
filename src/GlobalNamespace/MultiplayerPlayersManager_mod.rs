#[cfg(feature = "MultiplayerPlayersManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPlayersManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _activeLocalPlayerControllerPrefab: *mut MultiplayerLocalActivePlayerFacade,
    pub _activeLocalPlayerDuelControllerPrefab: *mut MultiplayerLocalActivePlayerFacade,
    pub _inactiveLocalPlayerControllerPrefab: *mut MultiplayerLocalInactivePlayerFacade,
    pub _connectedPlayerControllerPrefab: *mut MultiplayerConnectedPlayerFacade,
    pub _connectedPlayerDuelControllerPrefab: *mut MultiplayerConnectedPlayerFacade,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _beatmapObjectSpawnCenter: *mut BeatmapObjectSpawnCenter,
    pub _layoutProvider: *mut MultiplayerLayoutProvider,
    pub _fadeInOutController: *mut FadeInOutController,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _playerSpawningFinished_k__BackingField: bool,
    pub playerSpawningDidFinishEvent: *mut crate::System::Action,
    pub didSwitchPlayerToInactiveEvent: *mut crate::System::Action,
    pub playerDidFinishEvent: *mut crate::System::Action_1<
        *mut MultiplayerLevelCompletionResults,
    >,
    pub playerNetworkDidFailedEvent: *mut crate::System::Action_1<
        *mut MultiplayerLevelCompletionResults,
    >,
    pub _activeLocalPlayerFacade: *mut MultiplayerLocalActivePlayerFacade,
    pub _inactiveLocalPlayerFacade: *mut MultiplayerLocalInactivePlayerFacade,
    pub _currentEventsPublisher: *mut IMultiplayerLevelEndActionsPublisher,
    pub _currentStartSeekSongControllerProvider: *mut IStartSeekSongControllerProvider,
    pub _activeLocalPlayerFactory: *mut crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade_Factory,
    pub _inactiveLocalPlayerFactory: *mut crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory,
    pub _connectedPlayerFactory: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerFacade_Factory,
    pub _connectedPlayerControllersMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut MultiplayerConnectedPlayerFacade,
    >,
    pub _connectedPlayerCenterFacingRotationsMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        f32,
    >,
    pub _allActiveAtGameStartPlayers: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut IConnectedPlayer,
    >,
}
#[cfg(feature = "MultiplayerPlayersManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerPlayersManager => ""
    ."MultiplayerPlayersManager"
);
#[cfg(feature = "MultiplayerPlayersManager")]
impl std::ops::Deref for MultiplayerPlayersManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayersManager")]
impl std::ops::DerefMut for MultiplayerPlayersManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayersManager")]
impl MultiplayerPlayersManager {
    #[cfg(
        feature = "MultiplayerPlayersManager+_SwitchLocalPlayerToInactiveCoroutine_d__51"
    )]
    pub type _SwitchLocalPlayerToInactiveCoroutine_d__51 = crate::GlobalNamespace::MultiplayerPlayersManager__SwitchLocalPlayerToInactiveCoroutine_d__51;
    pub fn BindPlayerFactories(
        &mut self,
        layout: MultiplayerPlayerLayout,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindPlayerFactories", (layout))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerDidFinish(
        &mut self,
        levelCompletionResults: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerDidFinish", (levelCompletionResults))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerNetworkDidFailed(
        &mut self,
        levelCompletionResults: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerNetworkDidFailed", (levelCompletionResults))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ReportLocalPlayerNetworkDidFailed(
        &mut self,
        levelCompletionResults: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportLocalPlayerNetworkDidFailed", (levelCompletionResults))?;
        Ok(__cordl_ret)
    }
    pub fn SpawnPlayers(
        &mut self,
        localPlayerStartState: MultiplayerPlayerStartState,
        allActiveAtGameStartPlayers: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SpawnPlayers",
                (localPlayerStartState, allActiveAtGameStartPlayers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SwitchLocalPlayerToInactive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchLocalPlayerToInactive", ())?;
        Ok(__cordl_ret)
    }
    pub fn SwitchLocalPlayerToInactiveCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("SwitchLocalPlayerToInactiveCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryGetConnectedCenterFacingRotation(
        &mut self,
        userId: *mut crate::System::String,
        centerFacingRotation: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryGetConnectedCenterFacingRotation",
                (userId, centerFacingRotation),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetConnectedPlayerController(
        &mut self,
        userId: *mut crate::System::String,
        connectedPlayerController: quest_hook::libil2cpp::ByRefMut<
            *mut MultiplayerConnectedPlayerFacade,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryGetConnectedPlayerController",
                (userId, connectedPlayerController),
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
    pub fn add_didSwitchPlayerToInactiveEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSwitchPlayerToInactiveEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerNetworkDidFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerNetworkDidFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerSpawningDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerSpawningDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_activeLocalPlayerFacade(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerLocalActivePlayerFacade> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerLocalActivePlayerFacade = __cordl_object
            .invoke("get_activeLocalPlayerFacade", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_allActiveAtGameStartPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<*mut IConnectedPlayer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut IConnectedPlayer,
        > = __cordl_object.invoke("get_allActiveAtGameStartPlayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_inactivePlayerFacade(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerLocalInactivePlayerFacade> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerLocalInactivePlayerFacade = __cordl_object
            .invoke("get_inactivePlayerFacade", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayerStartSeekSongController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IStartSeekSongControllerProvider> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IStartSeekSongControllerProvider = __cordl_object
            .invoke("get_localPlayerStartSeekSongController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayerTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_localPlayerTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerSpawningFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_playerSpawningFinished", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSwitchPlayerToInactiveEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSwitchPlayerToInactiveEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerNetworkDidFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerNetworkDidFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerSpawningDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerSpawningDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerSpawningFinished(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerSpawningFinished", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerPlayersManager")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerPlayersManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
