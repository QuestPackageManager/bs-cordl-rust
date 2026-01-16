#[cfg(feature = "cordl_class_MainSystemInit")]
#[repr(C)]
#[derive(Debug)]
pub struct MainSystemInit {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _songPackMasksModel:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SongPackMasksModelSO>,
    pub _playerDataFileManager:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerDataFileManagerSO>,
    pub _standardLevelScenesTransitionSetupData:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO>,
    pub _missionLevelScenesTransitionSetupData:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO>,
    pub _multiplayerLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
    >,
    pub _playerDataModelPrefab: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerDataModel>,
    pub _campaignProgressModelPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CampaignProgressModel>,
    pub _customLevelLoaderPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CustomLevelLoader>,
    pub _multiplayerSessionManagerPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberMultiplayerSessionManager>,
    pub _voipManagerPrefab: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VoipManager>,
    pub _gameLiftNetworkPlayerModelPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameLiftNetworkPlayerModel>,
    pub _localServerNetworkPlayerModelPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalServerNetworkPlayerModel>,
    pub _networkPlayerEntitlementCheckerPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NetworkPlayerEntitlementChecker>,
    pub _tweeningManagerPrefab: quest_hook::libil2cpp::Gc<crate::Tweening::TimeTweeningManager>,
    pub _lightsUpdateSystemPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomPrePassLightsUpdateSystem>,
    pub _environmentAudioEffectsPlayerPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentAudioEffectsPlayer>,
    pub _nodePoseSyncStateManagerPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NodePoseSyncStateManager>,
    pub _unityXRHelperPrefab: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UnityXRHelper>,
    pub _devicelessVRHelperPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DevicelessVRHelper>,
    pub _richPresenceManagerPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::RichPresenceManager>,
    pub _dlcPromoPanelData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DlcPromoPanelDataSO>,
    pub _beatmapLevelsPromoData:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelsPromoDataSO>,
    pub _networkConfig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NetworkConfigSO>,
    pub _steamNetworkPlayerModelPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SteamNetworkPlayerModel>,
    pub _oculusNetworkPlayerModelPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OculusNetworkPlayerModel>,
    pub _leaderboardScoreUploader:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LeaderboardScoreUploader>,
    pub _platformLeaderboardsModel:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlatformLeaderboardsModel>,
    pub _coroutineStarter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CoroutineStarter>,
    pub _menuTransitionHelperPrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuTransitionsHelper>,
    pub _defaultMaxCachedBeatmapLevels: i32,
    pub _persistentAudioNoisePrefab:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PersistentAudioNoiseController>,
    pub _settingsManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsManager>,
}
#[cfg(feature = "cordl_class_MainSystemInit")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::MainSystemInit {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MainSystemInit";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "MainSystemInit")]
impl std::ops::Deref for crate::GlobalNamespace::MainSystemInit {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainSystemInit")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainSystemInit {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainSystemInit")]
impl crate::GlobalNamespace::MainSystemInit {
    pub fn Init(
        &mut self,
        settingsApplicator: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsApplicatorSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::SettingsApplicatorSO,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (settingsApplicator))? };
        Ok(__cordl_ret.into())
    }
    pub fn InstallBindings(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        isRunningFromTests: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("InstallBindings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InstallBindings", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (container, isRunningFromTests))? };
        Ok(__cordl_ret.into())
    }
    pub fn InstallOculusDestinationBindings(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InstallOculusDestinationBindings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InstallOculusDestinationBindings", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (container))? };
        Ok(__cordl_ret.into())
    }
    pub fn InstallPlatformLeaderboardsModel(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        isRunningFromTests: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("InstallPlatformLeaderboardsModel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InstallPlatformLeaderboardsModel", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (container, isRunningFromTests))? };
        Ok(__cordl_ret.into())
    }
    pub fn InstallRichPresence(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        isRunningFromTests: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("InstallRichPresence")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InstallRichPresence", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (container, isRunningFromTests))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_MainSystemInit")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MainSystemInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
