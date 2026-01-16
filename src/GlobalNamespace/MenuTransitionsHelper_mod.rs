#[cfg(feature = "cordl_class_MenuTransitionsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuTransitionsHelper {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _appInitScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO,
    >,
    pub _standardLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    >,
    pub _multiplayerLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
    >,
    pub _missionLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
    >,
    pub _tutorialScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO,
    >,
    pub _creditsScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
    >,
    pub _healthWarningScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO,
    >,
    pub _shaderWarmupSceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ShaderWarmupScenesTransitionSetupDataSO,
    >,
    pub _menuScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
    >,
    pub _startupErrorScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StartupErrorScenesTransitionSetupDataSO,
    >,
    pub _recordingToolScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RecordingToolScenesTransitionSetupDataSO,
    >,
    pub _gameScenesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameScenesManager,
    >,
    pub _beatmapDataLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataLoader,
    >,
    pub _beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
    >,
    pub _audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioClipAsyncLoader,
    >,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _telemetryModel: quest_hook::libil2cpp::Gc<
        crate::Analytics::Model::TelemetryModel,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub _standardLevelFinishedCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
        >,
    >,
    pub _standardLevelRestartedCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
        >,
    >,
    pub _multiplayerLevelFinishedCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerResultsData>,
        >,
    >,
    pub _multiplayerDidDisconnectCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
    >,
    pub _missionLevelFinishedCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionCompletionResults>,
        >,
    >,
    pub _missionLevelRestartedCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionCompletionResults>,
        >,
    >,
    pub _beatmapEditorFinishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _beatmapEditorGameplayLevelFinishedCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
        >,
    >,
}
#[cfg(feature = "cordl_class_MenuTransitionsHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MenuTransitionsHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MenuTransitionsHelper";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "MenuTransitionsHelper")]
impl std::ops::Deref for crate::GlobalNamespace::MenuTransitionsHelper {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuTransitionsHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuTransitionsHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuTransitionsHelper")]
impl crate::GlobalNamespace::MenuTransitionsHelper {
    pub fn HandleCreditsSceneDidFinish(
        &mut self,
        creditsSceneTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleCreditsSceneDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleCreditsSceneDidFinish", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (creditsSceneTransitionSetupData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMainGameSceneDidFinish(
        &mut self,
        standardLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        >,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCompletionResults,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleMainGameSceneDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleMainGameSceneDidFinish", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (standardLevelScenesTransitionSetupData, levelCompletionResults),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMissionLevelSceneDidFinish(
        &mut self,
        missionLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
        >,
        missionCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MissionCompletionResults,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleMissionLevelSceneDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleMissionLevelSceneDidFinish", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (missionLevelScenesTransitionSetupData, missionCompletionResults),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLevelDidDisconnect(
        &mut self,
        multiplayerLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        >,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                            >,
                            crate::GlobalNamespace::DisconnectedReason,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleMultiplayerLevelDidDisconnect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleMultiplayerLevelDidDisconnect", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (multiplayerLevelScenesTransitionSetupData, disconnectedReason),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLevelDidFinish(
        &mut self,
        multiplayerLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        >,
        multiplayerResultsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MultiplayerResultsData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleMultiplayerLevelDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleMultiplayerLevelDidFinish", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (multiplayerLevelScenesTransitionSetupData, multiplayerResultsData),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleTutorialSceneDidFinish(
        &mut self,
        tutorialSceneTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO,
        >,
        endState: crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO_TutorialEndStateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO,
                            >,
                            crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO_TutorialEndStateType,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleTutorialSceneDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleTutorialSceneDidFinish", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (tutorialSceneTransitionSetupData, endState))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnDestroy", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RestartGame(
        &mut self,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RestartGame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RestartGame", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (finishCallback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowCredits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ShowCredits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShowCredits", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartBeatmapEditor(
        &mut self,
        beatmapEditorFinishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("StartBeatmapEditor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartBeatmapEditor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (beatmapEditorFinishedCallback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartBeatmapEditorStandardLevel(
        &mut self,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        practiceSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        >,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        data: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapEditorStartTestLevelData,
        >,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        afterSceneSwitchCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        levelFinishedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapLevelData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::BeatmapKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSpecificSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PracticeSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ColorScheme,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::EnvironmentsListModel,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::BeatmapEditorStartTestLevelData,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::LevelCompletionResults,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("StartBeatmapEditorStandardLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartBeatmapEditorStandardLevel", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        beatmapLevelData,
                        beatmapKey,
                        beatmapLevel,
                        gameplayModifiers,
                        playerSpecificSettings,
                        practiceSettings,
                        colorScheme,
                        environmentsListModel,
                        data,
                        beforeSceneSwitchCallback,
                        afterSceneSwitchCallback,
                        levelFinishedCallback,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartMissionLevel(
        &mut self,
        missionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        playerOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        playerOverrideLightshowColors: bool,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        missionObjectives: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjective>,
            >,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        levelFinishedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionCompletionResults,
                >,
            >,
        >,
        levelRestartedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::BeatmapKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ColorScheme,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::MissionObjective,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSpecificSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::EnvironmentsListModel,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::MissionCompletionResults,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::MissionCompletionResults,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("StartMissionLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartMissionLevel", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        missionId,
                        beatmapKey,
                        beatmapLevel,
                        playerOverrideColorScheme,
                        playerOverrideLightshowColors,
                        gameplayModifiers,
                        missionObjectives,
                        playerSpecificSettings,
                        environmentsListModel,
                        beforeSceneSwitchCallback,
                        levelFinishedCallback,
                        levelRestartedCallback,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartMultiplayerLevel(
        &mut self,
        gameMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
        overrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        practiceSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        >,
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useTestNoteCutSoundEffects: bool,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        levelFinishedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerResultsData>,
            >,
        >,
        didDisconnectCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::BeatmapKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapLevelData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ColorScheme,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSpecificSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::EnvironmentsListModel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PracticeSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::MultiplayerResultsData,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    crate::GlobalNamespace::DisconnectedReason,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        14usize,
                    >("StartMultiplayerLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartMultiplayerLevel", 14usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        gameMode,
                        beatmapKey,
                        beatmapLevel,
                        beatmapLevelData,
                        overrideColorScheme,
                        gameplayModifiers,
                        playerSpecificSettings,
                        environmentsListModel,
                        practiceSettings,
                        backButtonText,
                        useTestNoteCutSoundEffects,
                        beforeSceneSwitchCallback,
                        levelFinishedCallback,
                        didDisconnectCallback,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartStandardLevel(
        &mut self,
        gameMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        overrideEnvironmentSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        >,
        playerOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        playerOverrideLightshowColors: bool,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        practiceSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        gameplayAdditionalInformation: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayAdditionalInformation,
        >,
        beforeSceneSwitchToGameplayCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action,
        >,
        afterSceneSwitchToGameplayCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        levelFinishedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
            >,
        >,
        levelRestartedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
            >,
        >,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::BeatmapKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OverrideEnvironmentSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ColorScheme,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSpecificSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PracticeSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::EnvironmentsListModel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayAdditionalInformation,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::LevelCompletionResults,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::LevelCompletionResults,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapLevelData,
                            >,
                            crate::System::Nullable_1<
                                crate::GlobalNamespace::RecordingToolManager_SetupData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        17usize,
                    >("StartStandardLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartStandardLevel", 17usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        gameMode,
                        beatmapKey,
                        beatmapLevel,
                        overrideEnvironmentSettings,
                        playerOverrideColorScheme,
                        playerOverrideLightshowColors,
                        gameplayModifiers,
                        playerSpecificSettings,
                        practiceSettings,
                        environmentsListModel,
                        gameplayAdditionalInformation,
                        beforeSceneSwitchToGameplayCallback,
                        afterSceneSwitchToGameplayCallback,
                        levelFinishedCallback,
                        levelRestartedCallback,
                        beatmapLevelData,
                        recordingToolData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTutorial(
        &mut self,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSpecificSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::EnvironmentsListModel,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("StartTutorial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartTutorial", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        playerSpecificSettings,
                        environmentsListModel,
                        beforeSceneSwitchCallback,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_appInitScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_appInitScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_appInitScenesTransitionSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_creditsScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_creditsScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_creditsScenesTransitionSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_healthWarningScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_healthWarningScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_healthWarningScenesTransitionSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_menuScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_menuScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_menuScenesTransitionSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_missionLevelScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_missionLevelScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_missionLevelScenesTransitionSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplayerLevelScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_multiplayerLevelScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_multiplayerLevelScenesTransitionSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_recordingToolScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::RecordingToolScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_recordingToolScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_recordingToolScenesTransitionSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_shaderWarmupSceneSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ShaderWarmupScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ShaderWarmupScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_shaderWarmupSceneSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_shaderWarmupSceneSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ShaderWarmupScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_standardLevelScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_standardLevelScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_standardLevelScenesTransitionSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_startupErrorScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StartupErrorScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::StartupErrorScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_startupErrorScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_startupErrorScenesTransitionSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StartupErrorScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_tutorialScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO,
                        >,
                        0usize,
                    >("get_tutorialScenesTransitionSetupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_tutorialScenesTransitionSetupData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_MenuTransitionsHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuTransitionsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
