#[cfg(feature = "MainSystemInit")]
#[repr(C)]
#[derive(Debug)]
pub struct MainSystemInit {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _songPackMasksModel: *mut crate::GlobalNamespace::SongPackMasksModelSO,
    pub _playerDataFileManager: *mut crate::GlobalNamespace::PlayerDataFileManagerSO,
    pub _standardLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    pub _missionLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
    pub _multiplayerLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
    pub _timeHelperPrefab: *mut crate::GlobalNamespace::TimeHelper,
    pub _playerDataModelPrefab: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _campaignProgressModelPrefab: *mut crate::GlobalNamespace::CampaignProgressModel,
    pub _customLevelLoaderPrefab: *mut crate::GlobalNamespace::CustomLevelLoader,
    pub _externalCamerasManagerPrefab: *mut crate::GlobalNamespace::ExternalCamerasManager,
    pub _multiplayerSessionManagerPrefab: *mut crate::GlobalNamespace::MultiplayerSessionManager,
    pub _voipManagerPrefab: *mut crate::GlobalNamespace::VoipManager,
    pub _gameLiftNetworkPlayerModelPrefab: *mut crate::GlobalNamespace::GameLiftNetworkPlayerModel,
    pub _networkPlayerEntitlementCheckerPrefab: *mut crate::GlobalNamespace::NetworkPlayerEntitlementChecker,
    pub _tweeningManagerPrefab: *mut crate::Tweening::TimeTweeningManager,
    pub _lightsUpdateSystemPrefab: *mut crate::GlobalNamespace::BloomPrePassLightsUpdateSystem,
    pub _environmentAudioEffectsPlayerPrefab: *mut crate::GlobalNamespace::EnvironmentAudioEffectsPlayer,
    pub _nodePoseSyncStateManagerPrefab: *mut crate::GlobalNamespace::NodePoseSyncStateManager,
    pub _psVRHelperPrefab: *mut crate::GlobalNamespace::PSVRHelper,
    pub _psVR2HelperPrefab: *mut crate::GlobalNamespace::PSVR2Helper,
    pub _oculusVRHelperPrefab: *mut crate::GlobalNamespace::OculusVRHelper,
    pub _unityXRHelperPrefab: *mut crate::GlobalNamespace::UnityXRHelper,
    pub _devicelessVRHelperPrefab: *mut crate::GlobalNamespace::DevicelessVRHelper,
    pub _richPresenceManagerPrefab: *mut crate::GlobalNamespace::RichPresenceManager,
    pub _dlcPromoPanelData: *mut crate::GlobalNamespace::DlcPromoPanelDataSO,
    pub _beatmapLevelsPromoData: *mut crate::GlobalNamespace::BeatmapLevelsPromoDataSO,
    pub _networkConfig: *mut crate::GlobalNamespace::NetworkConfigSO,
    pub _steamNetworkPlayerModelPrefab: *mut crate::GlobalNamespace::SteamNetworkPlayerModel,
    pub _oculusNetworkPlayerModelPrefab: *mut crate::GlobalNamespace::OculusNetworkPlayerModel,
    pub _sonyNetworkPlayerModelPrefab: *mut crate::GlobalNamespace::SonyNetworkPlayerModel,
    pub _leaderboardScoreUploader: *mut crate::GlobalNamespace::LeaderboardScoreUploader,
    pub _platformLeaderboardsModel: *mut crate::GlobalNamespace::PlatformLeaderboardsModel,
    pub _ps4AchievementIdsModel: *mut crate::GlobalNamespace::SonyAchievementIdsModelSO,
    pub _ps5AchievmentIdsModel: *mut crate::GlobalNamespace::SonyAchievementIdsModelSO,
    pub _achievementIdsModel: *mut crate::GlobalNamespace::AchievementIdsModelSO,
    pub _achievementsModel: *mut crate::GlobalNamespace::AchievementsModelSO,
    pub _ps5ActivityIdsModel: *mut crate::GlobalNamespace::PS5ActivityIdsModelSO,
    pub _coroutineStarter: *mut crate::GlobalNamespace::CoroutineStarter,
    pub _menuTransitionHelperPrefab: *mut crate::GlobalNamespace::MenuTransitionsHelper,
    pub _defaultMaxCachedBeatmapLevels: i32,
    pub _ps4MaxCachedBeatmapLevels: i32,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
    pub _graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
}
#[cfg(feature = "MainSystemInit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MainSystemInit => ""
    ."MainSystemInit"
);
#[cfg(feature = "MainSystemInit")]
impl std::ops::Deref for crate::GlobalNamespace::MainSystemInit {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainSystemInit")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainSystemInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainSystemInit")]
impl crate::GlobalNamespace::MainSystemInit {
    #[cfg(feature = "MainSystemInit+__c")]
    pub type __c = crate::GlobalNamespace::MainSystemInit___c;
    pub fn Init(
        &mut self,
        settingsApplicator: *mut crate::GlobalNamespace::SettingsApplicatorSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (settingsApplicator))?;
        Ok(__cordl_ret)
    }
    pub fn InstallBindings(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        isRunningFromTests: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", (container, isRunningFromTests))?;
        Ok(__cordl_ret)
    }
    pub fn InstallOculusDestinationBindings(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallOculusDestinationBindings", (container))?;
        Ok(__cordl_ret)
    }
    pub fn InstallPS4Bindings(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallPS4Bindings", (container))?;
        Ok(__cordl_ret)
    }
    pub fn InstallPS5Bindings(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallPS5Bindings", (container))?;
        Ok(__cordl_ret)
    }
    pub fn InstallPlatformLeaderboardsModel(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        isRunningFromTests: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InstallPlatformLeaderboardsModel",
                (container, isRunningFromTests),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstallRichPresence(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        isRunningFromTests: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallRichPresence", (container, isRunningFromTests))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "MainSystemInit")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MainSystemInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
