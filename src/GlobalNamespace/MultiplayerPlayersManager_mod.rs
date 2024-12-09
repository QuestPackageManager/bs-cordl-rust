#[cfg(feature = "MultiplayerPlayersManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPlayersManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _activeLocalPlayerControllerPrefab: *mut crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade,
    pub _activeLocalPlayerDuelControllerPrefab: *mut crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade,
    pub _inactiveLocalPlayerControllerPrefab: *mut crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade,
    pub _connectedPlayerControllerPrefab: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
    pub _connectedPlayerDuelControllerPrefab: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _beatmapObjectSpawnCenter: *mut crate::GlobalNamespace::BeatmapObjectSpawnCenter,
    pub _layoutProvider: *mut crate::GlobalNamespace::MultiplayerLayoutProvider,
    pub _fadeInOutController: *mut crate::GlobalNamespace::FadeInOutController,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _playerSpawningFinished_k__BackingField: bool,
    pub playerSpawningDidFinishEvent: *mut crate::System::Action,
    pub didSwitchPlayerToInactiveEvent: *mut crate::System::Action,
    pub playerDidFinishEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
    >,
    pub playerNetworkDidFailedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
    >,
    pub _activeLocalPlayerFacade: *mut crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade,
    pub _inactiveLocalPlayerFacade: *mut crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade,
    pub _currentEventsPublisher: *mut crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher,
    pub _currentStartSeekSongControllerProvider: *mut crate::GlobalNamespace::IStartSeekSongControllerProvider,
    pub _activeLocalPlayerFactory: *mut crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade_Factory,
    pub _inactiveLocalPlayerFactory: *mut crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory,
    pub _connectedPlayerFactory: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerFacade_Factory,
    pub _connectedPlayerControllersMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
    >,
    pub _connectedPlayerCenterFacingRotationsMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        f32,
    >,
    pub _allActiveAtGameStartPlayers: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::GlobalNamespace::IConnectedPlayer,
    >,
}
#[cfg(feature = "MultiplayerPlayersManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerPlayersManager => ""
    ."MultiplayerPlayersManager"
);
#[cfg(feature = "MultiplayerPlayersManager")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerPlayersManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayersManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerPlayersManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayersManager")]
impl crate::GlobalNamespace::MultiplayerPlayersManager {
    #[cfg(
        feature = "MultiplayerPlayersManager+_SwitchLocalPlayerToInactiveCoroutine_d__51"
    )]
    pub type _SwitchLocalPlayerToInactiveCoroutine_d__51 = crate::GlobalNamespace::MultiplayerPlayersManager__SwitchLocalPlayerToInactiveCoroutine_d__51;
    pub fn BindPlayerFactories(
        &mut self,
        layout: crate::GlobalNamespace::MultiplayerPlayerLayout,
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
        levelCompletionResults: *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
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
        levelCompletionResults: *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
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
        levelCompletionResults: *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
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
        localPlayerStartState: crate::GlobalNamespace::MultiplayerPlayerStartState,
        allActiveAtGameStartPlayers: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
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
        userId: *mut quest_hook::libil2cpp::Il2CppString,
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
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        connectedPlayerController: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
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
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
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
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
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
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade = __cordl_object
            .invoke("get_activeLocalPlayerFacade", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_allActiveAtGameStartPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        > = __cordl_object.invoke("get_allActiveAtGameStartPlayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_inactivePlayerFacade(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade = __cordl_object
            .invoke("get_inactivePlayerFacade", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayerStartSeekSongController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::IStartSeekSongControllerProvider,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IStartSeekSongControllerProvider = __cordl_object
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
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
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
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
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
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerPlayersManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
