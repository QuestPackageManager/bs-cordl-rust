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
        beatmapEditorStandardLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
        >,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapEditorSceneDidFinish(
        &mut self,
        beatmapEditorScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEditorScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapEditorSceneDidFinish",
                (beatmapEditorScenesTransitionSetupData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleCreditsSceneDidFinish(
        &mut self,
        creditsSceneTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCreditsSceneDidFinish", (creditsSceneTransitionSetupData))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMainGameSceneDidFinish",
                (standardLevelScenesTransitionSetupData, levelCompletionResults),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMissionLevelSceneDidFinish",
                (missionLevelScenesTransitionSetupData, missionCompletionResults),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerLevelDidDisconnect(
        &mut self,
        multiplayerLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
        >,
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelDidFinish",
                (multiplayerLevelScenesTransitionSetupData, multiplayerResultsData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleTutorialSceneDidFinish(
        &mut self,
        tutorialSceneTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO,
        >,
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
    pub fn RestartGame(
        &mut self,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestartGame", (finishCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowCredits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowCredits", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartBeatmapEditor(
        &mut self,
        beatmapEditorFinishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartBeatmapEditor", (beatmapEditorFinishedCallback))?;
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
        useFirstPersonFlyingController: bool,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        levelFinishedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::LevelCompletionResults,
            >,
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
                *mut crate::GlobalNamespace::MissionObjective,
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
                *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::MissionCompletionResults,
            >,
        >,
        levelRestartedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::MissionCompletionResults,
            >,
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
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
        >,
        levelFinishedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::MultiplayerResultsData,
            >,
        >,
        didDisconnectCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
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
                *mut crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::MultiplayerResultsData,
            >,
        >,
        didDisconnectCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
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
        Ok(__cordl_ret.into())
    }
    pub fn StartStandardLevel_IBeatmapLevelData_OverrideEnvironmentSettings_ColorScheme_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_EnvironmentsListModel_Il2CppString__cordl_bool_Action_Action_1_Action_2_Nullable_1_1(
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
        overrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
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
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
        >,
        levelFinishedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::LevelCompletionResults,
            >,
        >,
        levelRestartedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::LevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::LevelCompletionResults,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn StartStandardLevel_OverrideEnvironmentSettings_ColorScheme_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_EnvironmentsListModel_Il2CppString__cordl_bool_Action_Action_1_Action_2_Nullable_1_0(
        &mut self,
        gameMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        overrideEnvironmentSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        >,
        overrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
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
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
        >,
        levelFinishedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::LevelCompletionResults,
            >,
        >,
        levelRestartedCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::LevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::LevelCompletionResults,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn StartTutorial(
        &mut self,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartTutorial",
                (playerSpecificSettings, beforeSceneSwitchCallback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StopStandardLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopStandardLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _HandleBeatmapEditorSceneDidFinish_b__39_0(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<HandleBeatmapEditorSceneDidFinish>b__39_0", (container))?;
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
