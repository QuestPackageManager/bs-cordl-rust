#[cfg(feature = "MultiplayerPlayersManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPlayersManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _activeLocalPlayerControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade,
    >,
    pub _activeLocalPlayerDuelControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade,
    >,
    pub _inactiveLocalPlayerControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade,
    >,
    pub _connectedPlayerControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
    >,
    pub _connectedPlayerDuelControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
    >,
    pub _multiplayerSessionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
    pub _beatmapObjectSpawnCenter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectSpawnCenter,
    >,
    pub _layoutProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLayoutProvider,
    >,
    pub _fadeInOutController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FadeInOutController,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _playerSpawningFinished_k__BackingField: bool,
    pub playerSpawningDidFinishEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub didSwitchPlayerToInactiveEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub playerDidFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerLevelCompletionResults,
            >,
        >,
    >,
    pub playerNetworkDidFailedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerLevelCompletionResults,
            >,
        >,
    >,
    pub _activeLocalPlayerFacade: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade,
    >,
    pub _inactiveLocalPlayerFacade: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade,
    >,
    pub _currentEventsPublisher: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher,
    >,
    pub _currentStartSeekSongControllerProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IStartSeekSongControllerProvider,
    >,
    pub _activeLocalPlayerFactory: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade_Factory,
    >,
    pub _inactiveLocalPlayerFactory: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade_Factory,
    >,
    pub _connectedPlayerFactory: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerFacade_Factory,
    >,
    pub _connectedPlayerControllersMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
            >,
        >,
    >,
    pub _connectedPlayerCenterFacingRotationsMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            f32,
        >,
    >,
    pub _allActiveAtGameStartPlayers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        >,
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
    pub fn BindPlayerFactories(
        &mut self,
        layout: crate::GlobalNamespace::MultiplayerPlayerLayout,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindPlayerFactories", (layout))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerDidFinish(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerDidFinish", (levelCompletionResults))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerNetworkDidFailed(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerNetworkDidFailed", (levelCompletionResults))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReportLocalPlayerNetworkDidFailed(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportLocalPlayerNetworkDidFailed", (levelCompletionResults))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnPlayers(
        &mut self,
        localPlayerStartState: crate::GlobalNamespace::MultiplayerPlayerStartState,
        allActiveAtGameStartPlayers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn SwitchLocalPlayerToInactive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchLocalPlayerToInactive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchLocalPlayerToInactiveCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("SwitchLocalPlayerToInactiveCoroutine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetConnectedCenterFacingRotation(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn TryGetConnectedPlayerController(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        connectedPlayerController: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
            >,
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
    pub fn add_didSwitchPlayerToInactiveEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSwitchPlayerToInactiveEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_playerDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerDidFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_playerNetworkDidFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerNetworkDidFailedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_playerSpawningDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerSpawningDidFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeLocalPlayerFacade(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade,
        > = __cordl_object.invoke("get_activeLocalPlayerFacade", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allActiveAtGameStartPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        > = __cordl_object.invoke("get_allActiveAtGameStartPlayers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inactivePlayerFacade(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLocalInactivePlayerFacade,
        > = __cordl_object.invoke("get_inactivePlayerFacade", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localPlayerStartSeekSongController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IStartSeekSongControllerProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IStartSeekSongControllerProvider,
        > = __cordl_object.invoke("get_localPlayerStartSeekSongController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localPlayerTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("get_localPlayerTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerSpawningFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_playerSpawningFinished", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSwitchPlayerToInactiveEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSwitchPlayerToInactiveEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerDidFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerNetworkDidFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerNetworkDidFailedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerSpawningDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerSpawningDidFinishEvent", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "MultiplayerPlayersManager")]
impl AsRef<crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher>
for crate::GlobalNamespace::MultiplayerPlayersManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerPlayersManager")]
impl AsMut<crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher>
for crate::GlobalNamespace::MultiplayerPlayersManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IMultiplayerLevelEndActionsPublisher {
        unsafe { std::mem::transmute(self) }
    }
}
