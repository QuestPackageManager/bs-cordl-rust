#[cfg(feature = "RichPresenceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RichPresenceManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _standardLevelScenesTransitionSetupData: *mut StandardLevelScenesTransitionSetupDataSO,
    pub _tutorialScenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
    pub _missionLevelScenesTransitionSetupData: *mut MissionLevelScenesTransitionSetupDataSO,
    pub _multiplayerLevelScenesTransitionSetupData: *mut MultiplayerLevelScenesTransitionSetupDataSO,
    pub _menuScenesTransitionSetupData: *mut MenuScenesTransitionSetupDataSO,
    pub _richPresencePlatformHandler: *mut IRichPresencePlatformHandler,
    pub _gameScenesManager: *mut GameScenesManager,
    pub _lobbyGameStateModel: *mut LobbyGameStateModel,
    pub _unifiedNetworkPlayerModel: *mut IUnifiedNetworkPlayerModel,
    pub _lobbyPlayerPermissionsModel: *mut LobbyPlayerPermissionsModel,
    pub _menuWasLoaded: bool,
    pub _isInMultiplayerLobby: bool,
    pub _browsingMenusRichPresenceData: *mut BrowsingMenusRichPresenceData,
    pub _inMultiplayerRichPresenceData: *mut InMultiplayerRichPresenceData,
    pub _playingCampaignRichPresenceData: *mut PlayingCampaignRichPresenceData,
    pub _playingTutorialPresenceData: *mut PlayingTutorialPresenceData,
    pub _currentPresenceData: *mut IRichPresenceData,
}
#[cfg(feature = "RichPresenceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for RichPresenceManager => ""."RichPresenceManager"
);
#[cfg(feature = "RichPresenceManager")]
impl std::ops::Deref for RichPresenceManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RichPresenceManager")]
impl std::ops::DerefMut for RichPresenceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RichPresenceManager")]
impl RichPresenceManager {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleGameScenesManagerTransitionDidFinish(
        &mut self,
        scenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
        diContainer: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleGameScenesManagerTransitionDidFinish",
                (scenesTransitionSetupData, diContainer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateModelDidChange(
        &mut self,
        newGameState: MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateModelDidChange", (newGameState))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyPlayerPermissionChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyPlayerPermissionChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerPartySizeChanged(
        &mut self,
        currentPartySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplayerPartySizeChanged", (currentPartySize))?;
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
    pub fn SetMenuPresence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMenuPresence", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetPresence(
        &mut self,
        presenceData: *mut IRichPresenceData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPresence", (presenceData))?;
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
}
#[cfg(feature = "RichPresenceManager")]
impl quest_hook::libil2cpp::ObjectType for RichPresenceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
