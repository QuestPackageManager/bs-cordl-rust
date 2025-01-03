#[cfg(feature = "MainSystemInit")]
#[repr(C)]
#[derive(Debug)]
pub struct MainSystemInit {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _songPackMasksModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongPackMasksModelSO,
    >,
    pub _playerDataFileManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataFileManagerSO,
    >,
    pub _standardLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    >,
    pub _missionLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
    >,
    pub _multiplayerLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
    >,
    pub _timeHelperPrefab: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TimeHelper>,
    pub _playerDataModelPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _campaignProgressModelPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CampaignProgressModel,
    >,
    pub _customLevelLoaderPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CustomLevelLoader,
    >,
    pub _externalCamerasManagerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ExternalCamerasManager,
    >,
    pub _multiplayerSessionManagerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerSessionManager,
    >,
    pub _voipManagerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VoipManager,
    >,
    pub _gameLiftNetworkPlayerModelPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameLiftNetworkPlayerModel,
    >,
    pub _networkPlayerEntitlementCheckerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NetworkPlayerEntitlementChecker,
    >,
    pub _tweeningManagerPrefab: quest_hook::libil2cpp::Gc<
        crate::Tweening::TimeTweeningManager,
    >,
    pub _lightsUpdateSystemPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassLightsUpdateSystem,
    >,
    pub _environmentAudioEffectsPlayerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentAudioEffectsPlayer,
    >,
    pub _nodePoseSyncStateManagerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NodePoseSyncStateManager,
    >,
    pub _psVRHelperPrefab: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PSVRHelper>,
    pub _psVR2HelperPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PSVR2Helper,
    >,
    pub _oculusVRHelperPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OculusVRHelper,
    >,
    pub _unityXRHelperPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::UnityXRHelper,
    >,
    pub _devicelessVRHelperPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DevicelessVRHelper,
    >,
    pub _richPresenceManagerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RichPresenceManager,
    >,
    pub _dlcPromoPanelData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DlcPromoPanelDataSO,
    >,
    pub _beatmapLevelsPromoData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsPromoDataSO,
    >,
    pub _networkConfig: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NetworkConfigSO,
    >,
    pub _steamNetworkPlayerModelPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SteamNetworkPlayerModel,
    >,
    pub _oculusNetworkPlayerModelPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OculusNetworkPlayerModel,
    >,
    pub _sonyNetworkPlayerModelPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SonyNetworkPlayerModel,
    >,
    pub _leaderboardScoreUploader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LeaderboardScoreUploader,
    >,
    pub _platformLeaderboardsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlatformLeaderboardsModel,
    >,
    pub _ps4AchievementIdsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SonyAchievementIdsModelSO,
    >,
    pub _ps5AchievmentIdsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SonyAchievementIdsModelSO,
    >,
    pub _achievementIdsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementIdsModelSO,
    >,
    pub _achievementsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementsModelSO,
    >,
    pub _ps5ActivityIdsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PS5ActivityIdsModelSO,
    >,
    pub _coroutineStarter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CoroutineStarter,
    >,
    pub _menuTransitionHelperPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuTransitionsHelper,
    >,
    pub _defaultMaxCachedBeatmapLevels: i32,
    pub _ps4MaxCachedBeatmapLevels: i32,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
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
    pub fn Init(
        &mut self,
        settingsApplicator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsApplicatorSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (settingsApplicator))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallBindings(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        isRunningFromTests: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", (container, isRunningFromTests))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallOculusDestinationBindings(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallOculusDestinationBindings", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallPS4Bindings(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallPS4Bindings", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallPS5Bindings(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallPS5Bindings", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallPlatformLeaderboardsModel(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstallRichPresence(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        isRunningFromTests: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallRichPresence", (container, isRunningFromTests))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "MainSystemInit")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MainSystemInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
