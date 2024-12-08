#[cfg(feature = "MainSystemInit")]
#[repr(C)]
#[derive(Debug)]
pub struct MainSystemInit {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _songPackMasksModel: *mut SongPackMasksModelSO,
    pub _playerDataFileManager: *mut PlayerDataFileManagerSO,
    pub _standardLevelScenesTransitionSetupData: *mut StandardLevelScenesTransitionSetupDataSO,
    pub _missionLevelScenesTransitionSetupData: *mut MissionLevelScenesTransitionSetupDataSO,
    pub _multiplayerLevelScenesTransitionSetupData: *mut MultiplayerLevelScenesTransitionSetupDataSO,
    pub _timeHelperPrefab: *mut TimeHelper,
    pub _playerDataModelPrefab: *mut PlayerDataModel,
    pub _campaignProgressModelPrefab: *mut CampaignProgressModel,
    pub _customLevelLoaderPrefab: *mut CustomLevelLoader,
    pub _externalCamerasManagerPrefab: *mut ExternalCamerasManager,
    pub _multiplayerSessionManagerPrefab: *mut MultiplayerSessionManager,
    pub _voipManagerPrefab: *mut VoipManager,
    pub _gameLiftNetworkPlayerModelPrefab: *mut GameLiftNetworkPlayerModel,
    pub _networkPlayerEntitlementCheckerPrefab: *mut NetworkPlayerEntitlementChecker,
    pub _tweeningManagerPrefab: *mut crate::Tweening::TimeTweeningManager,
    pub _lightsUpdateSystemPrefab: *mut BloomPrePassLightsUpdateSystem,
    pub _environmentAudioEffectsPlayerPrefab: *mut EnvironmentAudioEffectsPlayer,
    pub _nodePoseSyncStateManagerPrefab: *mut NodePoseSyncStateManager,
    pub _psVRHelperPrefab: *mut PSVRHelper,
    pub _psVR2HelperPrefab: *mut PSVR2Helper,
    pub _oculusVRHelperPrefab: *mut OculusVRHelper,
    pub _unityXRHelperPrefab: *mut UnityXRHelper,
    pub _devicelessVRHelperPrefab: *mut DevicelessVRHelper,
    pub _richPresenceManagerPrefab: *mut RichPresenceManager,
    pub _dlcPromoPanelData: *mut DlcPromoPanelDataSO,
    pub _beatmapLevelsPromoData: *mut BeatmapLevelsPromoDataSO,
    pub _networkConfig: *mut NetworkConfigSO,
    pub _steamNetworkPlayerModelPrefab: *mut SteamNetworkPlayerModel,
    pub _oculusNetworkPlayerModelPrefab: *mut OculusNetworkPlayerModel,
    pub _sonyNetworkPlayerModelPrefab: *mut SonyNetworkPlayerModel,
    pub _leaderboardScoreUploader: *mut LeaderboardScoreUploader,
    pub _platformLeaderboardsModel: *mut PlatformLeaderboardsModel,
    pub _ps4AchievementIdsModel: *mut SonyAchievementIdsModelSO,
    pub _ps5AchievmentIdsModel: *mut SonyAchievementIdsModelSO,
    pub _achievementIdsModel: *mut AchievementIdsModelSO,
    pub _achievementsModel: *mut AchievementsModelSO,
    pub _ps5ActivityIdsModel: *mut PS5ActivityIdsModelSO,
    pub _coroutineStarter: *mut CoroutineStarter,
    pub _menuTransitionHelperPrefab: *mut MenuTransitionsHelper,
    pub _defaultMaxCachedBeatmapLevels: i32,
    pub _ps4MaxCachedBeatmapLevels: i32,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
    pub _graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
}
#[cfg(feature = "MainSystemInit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MainSystemInit => ""."MainSystemInit"
);
#[cfg(feature = "MainSystemInit")]
impl std::ops::Deref for MainSystemInit {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainSystemInit")]
impl std::ops::DerefMut for MainSystemInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainSystemInit")]
impl MainSystemInit {
    #[cfg(feature = "MainSystemInit+__c")]
    pub type __c = crate::GlobalNamespace::MainSystemInit___c;
    pub fn Init(
        &mut self,
        settingsApplicator: *mut SettingsApplicatorSO,
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
impl quest_hook::libil2cpp::ObjectType for MainSystemInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
