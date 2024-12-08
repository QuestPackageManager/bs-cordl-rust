#[cfg(feature = "MenuTransitionsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuTransitionsHelper {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _appInitScenesTransitionSetupDataContainer: *mut crate::GlobalNamespace::AppInitScenesTransitionSetupDataContainerSO,
    pub _standardLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    pub _multiplayerLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
    pub _missionLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
    pub _tutorialScenesTransitionSetupData: *mut crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO,
    pub _creditsScenesTransitionSetupData: *mut crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
    pub _gameScenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    pub _beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
    pub _beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
    pub _audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
    pub _graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
    pub _beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub _standardLevelFinishedCallback: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        *mut crate::GlobalNamespace::LevelCompletionResults,
    >,
    pub _standardLevelRestartedCallback: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::LevelScenesTransitionSetupDataSO,
        *mut crate::GlobalNamespace::LevelCompletionResults,
    >,
    pub _multiplayerLevelFinishedCallback: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        *mut crate::GlobalNamespace::MultiplayerResultsData,
    >,
    pub _multiplayerDidDisconnectCallback: *mut crate::System::Action_1<
        crate::GlobalNamespace::DisconnectedReason,
    >,
    pub _missionLevelFinishedCallback: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
        *mut crate::GlobalNamespace::MissionCompletionResults,
    >,
    pub _missionLevelRestartedCallback: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
        *mut crate::GlobalNamespace::MissionCompletionResults,
    >,
    pub _beatmapEditorFinishedCallback: *mut crate::System::Action,
    pub _beatmapEditorGameplayLevelFinishedCallback: *mut crate::System::Action_2<
        *mut crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
        *mut crate::GlobalNamespace::LevelCompletionResults,
    >,
}
#[cfg(feature = "MenuTransitionsHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuTransitionsHelper => ""
    ."MenuTransitionsHelper"
);
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
    #[cfg(feature = "MenuTransitionsHelper+__c__DisplayClass33_0")]
    pub type __c__DisplayClass33_0 = crate::GlobalNamespace::MenuTransitionsHelper___c__DisplayClass33_0;
    #[cfg(feature = "MenuTransitionsHelper+__c__DisplayClass34_0")]
    pub type __c__DisplayClass34_0 = crate::GlobalNamespace::MenuTransitionsHelper___c__DisplayClass34_0;
    #[cfg(feature = "MenuTransitionsHelper+__c__DisplayClass35_0")]
    pub type __c__DisplayClass35_0 = crate::GlobalNamespace::MenuTransitionsHelper___c__DisplayClass35_0;
    #[cfg(feature = "MenuTransitionsHelper+__c__DisplayClass36_0")]
    pub type __c__DisplayClass36_0 = crate::GlobalNamespace::MenuTransitionsHelper___c__DisplayClass36_0;
    #[cfg(feature = "MenuTransitionsHelper+__c__DisplayClass37_0")]
    pub type __c__DisplayClass37_0 = crate::GlobalNamespace::MenuTransitionsHelper___c__DisplayClass37_0;
    #[cfg(feature = "MenuTransitionsHelper+__c__DisplayClass40_0")]
    pub type __c__DisplayClass40_0 = crate::GlobalNamespace::MenuTransitionsHelper___c__DisplayClass40_0;
    pub fn HandleBeatmapEditorGameSceneDidFinish(
        &mut self,
        beatmapEditorStandardLevelScenesTransitionSetupData: *mut crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
        levelCompletionResults: *mut crate::GlobalNamespace::LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapEditorGameSceneDidFinish",
                (
                    beatmapEditorStandardLevelScenesTransitionSetupData,
                    levelCompletionResults,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapEditorSceneDidFinish(
        &mut self,
        beatmapEditorScenesTransitionSetupData: *mut crate::GlobalNamespace::BeatmapEditorScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapEditorSceneDidFinish",
                (beatmapEditorScenesTransitionSetupData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleCreditsSceneDidFinish(
        &mut self,
        creditsSceneTransitionSetupData: *mut crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCreditsSceneDidFinish", (creditsSceneTransitionSetupData))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMainGameSceneDidFinish(
        &mut self,
        standardLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        levelCompletionResults: *mut crate::GlobalNamespace::LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMainGameSceneDidFinish",
                (standardLevelScenesTransitionSetupData, levelCompletionResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMissionLevelSceneDidFinish(
        &mut self,
        missionLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
        missionCompletionResults: *mut crate::GlobalNamespace::MissionCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMissionLevelSceneDidFinish",
                (missionLevelScenesTransitionSetupData, missionCompletionResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLevelDidDisconnect(
        &mut self,
        multiplayerLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelDidDisconnect",
                (multiplayerLevelScenesTransitionSetupData, disconnectedReason),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLevelDidFinish(
        &mut self,
        multiplayerLevelScenesTransitionSetupData: *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        multiplayerResultsData: *mut crate::GlobalNamespace::MultiplayerResultsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelDidFinish",
                (multiplayerLevelScenesTransitionSetupData, multiplayerResultsData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleTutorialSceneDidFinish(
        &mut self,
        tutorialSceneTransitionSetupData: *mut crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO,
        endState: crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO_TutorialEndStateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleTutorialSceneDidFinish",
                (tutorialSceneTransitionSetupData, endState),
            )?;
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
    pub fn RestartGame(
        &mut self,
        finishCallback: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestartGame", (finishCallback))?;
        Ok(__cordl_ret)
    }
    pub fn ShowCredits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowCredits", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartBeatmapEditor(
        &mut self,
        beatmapEditorFinishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartBeatmapEditor", (beatmapEditorFinishedCallback))?;
        Ok(__cordl_ret)
    }
    pub fn StartBeatmapEditorStandardLevel(
        &mut self,
        beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        environmentsListModel: *mut crate::GlobalNamespace::EnvironmentsListModel,
        useFirstPersonFlyingController: bool,
        beforeSceneSwitchCallback: *mut crate::System::Action,
        levelFinishedCallback: *mut crate::System::Action_2<
            *mut crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartBeatmapEditorStandardLevel",
                (
                    beatmapLevelData,
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    colorScheme,
                    environmentsListModel,
                    useFirstPersonFlyingController,
                    beforeSceneSwitchCallback,
                    levelFinishedCallback,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartMissionLevel(
        &mut self,
        missionId: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        overrideColorScheme: *mut crate::GlobalNamespace::ColorScheme,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MissionObjective,
        >,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        environmentsListModel: *mut crate::GlobalNamespace::EnvironmentsListModel,
        beforeSceneSwitchCallback: *mut crate::System::Action,
        levelFinishedCallback: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::MissionCompletionResults,
        >,
        levelRestartedCallback: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::MissionCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartMissionLevel",
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartMultiplayerLevel_Action_1_Action_2_Action_1_1(
        &mut self,
        gameMode: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
        overrideColorScheme: *mut crate::GlobalNamespace::ColorScheme,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        backButtonText: *mut crate::System::String,
        useTestNoteCutSoundEffects: bool,
        beforeSceneSwitchCallback: *mut crate::System::Action,
        afterSceneSwitchCallback: *mut crate::System::Action_1<
            *mut crate::Zenject::DiContainer,
        >,
        levelFinishedCallback: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::MultiplayerResultsData,
        >,
        didDisconnectCallback: *mut crate::System::Action_1<
            crate::GlobalNamespace::DisconnectedReason,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartMultiplayerLevel",
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartMultiplayerLevel_Action_2_Action_1_0(
        &mut self,
        gameMode: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
        overrideColorScheme: *mut crate::GlobalNamespace::ColorScheme,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        backButtonText: *mut crate::System::String,
        useTestNoteCutSoundEffects: bool,
        beforeSceneSwitchCallback: *mut crate::System::Action,
        levelFinishedCallback: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::MultiplayerResultsData,
        >,
        didDisconnectCallback: *mut crate::System::Action_1<
            crate::GlobalNamespace::DisconnectedReason,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartMultiplayerLevel",
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartStandardLevel_IBeatmapLevelData_OverrideEnvironmentSettings_ColorScheme_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_EnvironmentsListModel_String__cordl_bool_Action_Action_1_Action_2_Nullable_1_1(
        &mut self,
        gameMode: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
        overrideEnvironmentSettings: *mut crate::GlobalNamespace::OverrideEnvironmentSettings,
        overrideColorScheme: *mut crate::GlobalNamespace::ColorScheme,
        beatmapOverrideColorScheme: *mut crate::GlobalNamespace::ColorScheme,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        environmentsListModel: *mut crate::GlobalNamespace::EnvironmentsListModel,
        backButtonText: *mut crate::System::String,
        useTestNoteCutSoundEffects: bool,
        startPaused: bool,
        beforeSceneSwitchCallback: *mut crate::System::Action,
        afterSceneSwitchCallback: *mut crate::System::Action_1<
            *mut crate::Zenject::DiContainer,
        >,
        levelFinishedCallback: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::LevelCompletionResults,
        >,
        levelRestartedCallback: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::LevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::LevelCompletionResults,
        >,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartStandardLevel",
                (
                    gameMode,
                    beatmapKey,
                    beatmapLevel,
                    beatmapLevelData,
                    overrideEnvironmentSettings,
                    overrideColorScheme,
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartStandardLevel_OverrideEnvironmentSettings_ColorScheme_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_EnvironmentsListModel_String__cordl_bool_Action_Action_1_Action_2_Nullable_1_0(
        &mut self,
        gameMode: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        overrideEnvironmentSettings: *mut crate::GlobalNamespace::OverrideEnvironmentSettings,
        overrideColorScheme: *mut crate::GlobalNamespace::ColorScheme,
        beatmapOverrideColorScheme: *mut crate::GlobalNamespace::ColorScheme,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        environmentsListModel: *mut crate::GlobalNamespace::EnvironmentsListModel,
        backButtonText: *mut crate::System::String,
        useTestNoteCutSoundEffects: bool,
        startPaused: bool,
        beforeSceneSwitchCallback: *mut crate::System::Action,
        afterSceneSwitchCallback: *mut crate::System::Action_1<
            *mut crate::Zenject::DiContainer,
        >,
        levelFinishedCallback: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::LevelCompletionResults,
        >,
        levelRestartedCallback: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::LevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::LevelCompletionResults,
        >,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartStandardLevel",
                (
                    gameMode,
                    beatmapKey,
                    beatmapLevel,
                    overrideEnvironmentSettings,
                    overrideColorScheme,
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTutorial(
        &mut self,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        beforeSceneSwitchCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartTutorial",
                (playerSpecificSettings, beforeSceneSwitchCallback),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StopStandardLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopStandardLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn _HandleBeatmapEditorSceneDidFinish_b__39_0(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<HandleBeatmapEditorSceneDidFinish>b__39_0", (container))?;
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
