#[cfg(feature = "MenuTransitionsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuTransitionsHelper {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _appInitScenesTransitionSetupDataContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AppInitScenesTransitionSetupDataContainerSO,
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
#[cfg(feature = "MenuTransitionsHelper")]
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
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuTransitionsHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuTransitionsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuTransitionsHelper")]
impl crate::GlobalNamespace::MenuTransitionsHelper {
    pub fn HandleBeatmapEditorGameSceneDidFinish(
        &mut self,
        beatmapEditorStandardLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
        >,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCompletionResults,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleBeatmapEditorGameSceneDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HandleBeatmapEditorGameSceneDidFinish",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        beatmapEditorStandardLevelScenesTransitionSetupData,
                        levelCompletionResults,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapEditorSceneDidFinish(
        &mut self,
        beatmapEditorScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEditorScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEditorScenesTransitionSetupDataSO,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleBeatmapEditorSceneDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HandleBeatmapEditorSceneDidFinish", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beatmapEditorScenesTransitionSetupData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleCreditsSceneDidFinish(
        &mut self,
        creditsSceneTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                            Self::class(), "HandleCreditsSceneDidFinish", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (creditsSceneTransitionSetupData))?
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                            Self::class(), "HandleMainGameSceneDidFinish", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                            Self::class(), "HandleMissionLevelSceneDidFinish", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                            Self::class(), "HandleMultiplayerLevelDidDisconnect", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                            Self::class(), "HandleMultiplayerLevelDidFinish", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                            Self::class(), "HandleTutorialSceneDidFinish", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tutorialSceneTransitionSetupData, endState))?
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OnDestroy", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                            Self::class(), "RestartGame", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (finishCallback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowCredits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ShowCredits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShowCredits", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartBeatmapEditor(
        &mut self,
        beatmapEditorFinishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("StartBeatmapEditor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StartBeatmapEditor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beatmapEditorFinishedCallback))?
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                        11usize,
                    >("StartBeatmapEditorStandardLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StartBeatmapEditorStandardLevel", 11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
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
        overrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                        11usize,
                    >("StartMissionLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StartMissionLevel", 11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        missionId,
                        beatmapKey,
                        beatmapLevel,
                        overrideColorScheme,
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
    pub fn StartMultiplayerLevel_Action_1_Action_2_Action_1_1(
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
        practiceSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        >,
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useTestNoteCutSoundEffects: bool,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        afterSceneSwitchCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                                crate::GlobalNamespace::PracticeSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                >,
                            >,
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
                            Self::class(), "StartMultiplayerLevel", 14usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
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
                        practiceSettings,
                        backButtonText,
                        useTestNoteCutSoundEffects,
                        beforeSceneSwitchCallback,
                        afterSceneSwitchCallback,
                        levelFinishedCallback,
                        didDisconnectCallback,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartMultiplayerLevel_Action_2_Action_1_0(
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                        13usize,
                    >("StartMultiplayerLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StartMultiplayerLevel", 13usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
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
    pub fn StartStandardLevel_IBeatmapLevelData_OverrideEnvironmentSettings_ColorScheme__cordl_bool_ColorScheme_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_EnvironmentsListModel_Il2CppString__cordl_bool_Action_Action_1_Action_2_Nullable_1_1(
        &mut self,
        gameMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
        overrideEnvironmentSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        >,
        playerOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        playerOverrideLightshowColors: bool,
        beatmapOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
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
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useTestNoteCutSoundEffects: bool,
        startPaused: bool,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        afterSceneSwitchCallback: quest_hook::libil2cpp::Gc<
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
                    crate::GlobalNamespace::LevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
            >,
        >,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                                crate::GlobalNamespace::OverrideEnvironmentSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ColorScheme,
                            >,
                            bool,
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
                                crate::GlobalNamespace::PracticeSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::EnvironmentsListModel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            bool,
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
                                        crate::GlobalNamespace::LevelScenesTransitionSetupDataSO,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::LevelCompletionResults,
                                    >,
                                >,
                            >,
                            crate::System::Nullable_1<
                                crate::GlobalNamespace::RecordingToolManager_SetupData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        20usize,
                    >("StartStandardLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StartStandardLevel", 20usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        gameMode,
                        beatmapKey,
                        beatmapLevel,
                        beatmapLevelData,
                        overrideEnvironmentSettings,
                        playerOverrideColorScheme,
                        playerOverrideLightshowColors,
                        beatmapOverrideColorScheme,
                        gameplayModifiers,
                        playerSpecificSettings,
                        practiceSettings,
                        environmentsListModel,
                        backButtonText,
                        useTestNoteCutSoundEffects,
                        startPaused,
                        beforeSceneSwitchCallback,
                        afterSceneSwitchCallback,
                        levelFinishedCallback,
                        levelRestartedCallback,
                        recordingToolData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartStandardLevel_OverrideEnvironmentSettings_ColorScheme__cordl_bool_ColorScheme_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_EnvironmentsListModel_Il2CppString__cordl_bool_Action_Action_1_Action_2_Nullable_1_0(
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
        beatmapOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
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
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useTestNoteCutSoundEffects: bool,
        startPaused: bool,
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
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                                crate::GlobalNamespace::ColorScheme,
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
                                crate::GlobalNamespace::EnvironmentsListModel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            bool,
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
                            crate::System::Nullable_1<
                                crate::GlobalNamespace::RecordingToolManager_SetupData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        19usize,
                    >("StartStandardLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StartStandardLevel", 19usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        gameMode,
                        beatmapKey,
                        beatmapLevel,
                        overrideEnvironmentSettings,
                        playerOverrideColorScheme,
                        playerOverrideLightshowColors,
                        beatmapOverrideColorScheme,
                        gameplayModifiers,
                        playerSpecificSettings,
                        practiceSettings,
                        environmentsListModel,
                        backButtonText,
                        useTestNoteCutSoundEffects,
                        startPaused,
                        beforeSceneSwitchToGameplayCallback,
                        afterSceneSwitchToGameplayCallback,
                        levelFinishedCallback,
                        levelRestartedCallback,
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
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSpecificSettings,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("StartTutorial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StartTutorial", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (playerSpecificSettings, beforeSceneSwitchCallback),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StopStandardLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("StopStandardLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StopStandardLevel", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _HandleBeatmapEditorSceneDidFinish_b__38_0(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("<HandleBeatmapEditorSceneDidFinish>b__38_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "<HandleBeatmapEditorSceneDidFinish>b__38_0",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MenuTransitionsHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuTransitionsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
