#[cfg(feature = "RichPresenceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RichPresenceManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _standardLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    pub _tutorialScenesTransitionSetupData: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    pub _missionLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
    pub _multiplayerLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
    pub _menuScenesTransitionSetupData: *mut crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
    pub _richPresencePlatformHandler: *mut crate::GlobalNamespace::IRichPresencePlatformHandler,
    pub _gameScenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    pub _lobbyGameStateModel: *mut crate::GlobalNamespace::LobbyGameStateModel,
    pub _unifiedNetworkPlayerModel: *mut crate::GlobalNamespace::IUnifiedNetworkPlayerModel,
    pub _lobbyPlayerPermissionsModel: *mut crate::GlobalNamespace::LobbyPlayerPermissionsModel,
    pub _menuWasLoaded: bool,
    pub _isInMultiplayerLobby: bool,
    pub _browsingMenusRichPresenceData: *mut crate::GlobalNamespace::BrowsingMenusRichPresenceData,
    pub _inMultiplayerRichPresenceData: *mut crate::GlobalNamespace::InMultiplayerRichPresenceData,
    pub _playingCampaignRichPresenceData: *mut crate::GlobalNamespace::PlayingCampaignRichPresenceData,
    pub _playingTutorialPresenceData: *mut crate::GlobalNamespace::PlayingTutorialPresenceData,
    pub _currentPresenceData: *mut crate::GlobalNamespace::IRichPresenceData,
}
#[cfg(feature = "RichPresenceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RichPresenceManager => ""
    ."RichPresenceManager"
);
#[cfg(feature = "RichPresenceManager")]
impl std::ops::Deref for crate::GlobalNamespace::RichPresenceManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RichPresenceManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::RichPresenceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RichPresenceManager")]
impl crate::GlobalNamespace::RichPresenceManager {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameScenesManagerTransitionDidFinish(
        &mut self,
        scenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        diContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleGameScenesManagerTransitionDidFinish",
                (scenesTransitionSetupData, diContainer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyGameStateModelDidChange(
        &mut self,
        newGameState: crate::GlobalNamespace::MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateModelDidChange", (newGameState))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyPlayerPermissionChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyPlayerPermissionChanged", ())?;
        Ok(__cordl_ret.into())
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
    pub fn SetMenuPresence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMenuPresence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPresence(
        &mut self,
        presenceData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IRichPresenceData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPresence", (presenceData))?;
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
}
#[cfg(feature = "RichPresenceManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RichPresenceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
