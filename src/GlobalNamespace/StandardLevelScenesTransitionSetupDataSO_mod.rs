#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelScenesTransitionSetupDataSO {
    __cordl_parent: LevelScenesTransitionSetupDataSO,
    pub _standardGameplaySceneInfo: *mut SceneInfo,
    pub _gameCoreSceneInfo: *mut SceneInfo,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut StandardLevelScenesTransitionSetupDataSO,
        *mut LevelCompletionResults,
    >,
    pub _gameMode_k__BackingField: *mut crate::System::String,
    pub _beatmapKey_k__BackingField: BeatmapKey,
    pub _beatmapLevel_k__BackingField: *mut BeatmapLevel,
    pub _practiceSettings_k__BackingField: *mut PracticeSettings,
    pub _usingOverrideColorScheme_k__BackingField: bool,
    pub _colorScheme_k__BackingField: *mut ColorScheme,
    pub _usingOverrideEnvironment_k__BackingField: bool,
    pub _environmentInfo_k__BackingField: *mut EnvironmentInfoSO,
    pub _gameplayModifiers_k__BackingField: *mut GameplayModifiers,
}
#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for StandardLevelScenesTransitionSetupDataSO => ""
    ."StandardLevelScenesTransitionSetupDataSO"
);
#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
impl std::ops::Deref for StandardLevelScenesTransitionSetupDataSO {
    type Target = LevelScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for StandardLevelScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
impl StandardLevelScenesTransitionSetupDataSO {
    pub fn Finish(
        &mut self,
        levelCompletionResults: *mut LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", (levelCompletionResults))?;
        Ok(__cordl_ret)
    }
    pub fn InitAndSetupScenes(
        &mut self,
        playerSpecificSettings: *mut PlayerSpecificSettings,
        backButtonText: *mut crate::System::String,
        startPaused: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitAndSetupScenes",
                (playerSpecificSettings, backButtonText, startPaused),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InitColorInfo(
        &mut self,
        overrideColorScheme: *mut ColorScheme,
        beatmapOverrideColorScheme: *mut ColorScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitColorInfo", (overrideColorScheme, beatmapOverrideColorScheme))?;
        Ok(__cordl_ret)
    }
    pub fn InitEnvironmentInfo(
        &mut self,
        overrideEnvironmentSettings: *mut OverrideEnvironmentSettings,
        environmentsListModel: *mut EnvironmentsListModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitEnvironmentInfo",
                (overrideEnvironmentSettings, environmentsListModel),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init_ByRefMut_BeatmapLevel_OverrideEnvironmentSettings_ColorScheme_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_EnvironmentsListModel_AudioClipAsyncLoader_BeatmapDataLoader_String_BeatmapLevelsModel_BeatmapLevelsEntitlementModel1(
        &mut self,
        gameMode: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        overrideEnvironmentSettings: *mut OverrideEnvironmentSettings,
        overrideColorScheme: *mut ColorScheme,
        beatmapOverrideColorScheme: *mut ColorScheme,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
        practiceSettings: *mut PracticeSettings,
        environmentsListModel: *mut EnvironmentsListModel,
        audioClipAsyncLoader: *mut AudioClipAsyncLoader,
        beatmapDataLoader: *mut BeatmapDataLoader,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        backButtonText: *mut crate::System::String,
        beatmapLevelsModel: *mut BeatmapLevelsModel,
        beatmapLevelsEntitlementModel: *mut BeatmapLevelsEntitlementModel,
        useTestNoteCutSoundEffects: bool,
        startPaused: bool,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
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
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    performancePreset,
                    backButtonText,
                    beatmapLevelsModel,
                    beatmapLevelsEntitlementModel,
                    useTestNoteCutSoundEffects,
                    startPaused,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init_IBeatmapLevelData_ByRefMut_BeatmapLevel_OverrideEnvironmentSettings_ColorScheme_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_EnvironmentsListModel_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel_String0(
        &mut self,
        gameMode: *mut crate::System::String,
        beatmapLevelData: *mut IBeatmapLevelData,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        overrideEnvironmentSettings: *mut OverrideEnvironmentSettings,
        overrideColorScheme: *mut ColorScheme,
        beatmapOverrideColorScheme: *mut ColorScheme,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
        practiceSettings: *mut PracticeSettings,
        environmentsListModel: *mut EnvironmentsListModel,
        audioClipAsyncLoader: *mut AudioClipAsyncLoader,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        beatmapDataLoader: *mut BeatmapDataLoader,
        beatmapLevelsEntitlementModel: *mut BeatmapLevelsEntitlementModel,
        backButtonText: *mut crate::System::String,
        useTestNoteCutSoundEffects: bool,
        startPaused: bool,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    gameMode,
                    beatmapLevelData,
                    beatmapKey,
                    beatmapLevel,
                    overrideEnvironmentSettings,
                    overrideColorScheme,
                    beatmapOverrideColorScheme,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    environmentsListModel,
                    audioClipAsyncLoader,
                    performancePreset,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    backButtonText,
                    useTestNoteCutSoundEffects,
                    startPaused,
                    recordingToolData,
                ),
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
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut StandardLevelScenesTransitionSetupDataSO,
            *mut LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapKey(&mut self) -> quest_hook::libil2cpp::Result<BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapKey = __cordl_object.invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevel = __cordl_object
            .invoke("get_beatmapLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut ColorScheme> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ColorScheme = __cordl_object
            .invoke("get_colorScheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut EnvironmentInfoSO = __cordl_object
            .invoke("get_environmentInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_gameMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut GameplayModifiers> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut GameplayModifiers = __cordl_object
            .invoke("get_gameplayModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_practiceSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PracticeSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PracticeSettings = __cordl_object
            .invoke("get_practiceSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_usingOverrideColorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_usingOverrideColorScheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_usingOverrideEnvironment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_usingOverrideEnvironment", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut StandardLevelScenesTransitionSetupDataSO,
            *mut LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapKey(
        &mut self,
        value: BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapKey", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapLevel(
        &mut self,
        value: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapLevel", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_colorScheme(
        &mut self,
        value: *mut ColorScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorScheme", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_environmentInfo(
        &mut self,
        value: *mut EnvironmentInfoSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_environmentInfo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_gameMode(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_gameplayModifiers(
        &mut self,
        value: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameplayModifiers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_practiceSettings(
        &mut self,
        value: *mut PracticeSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_practiceSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_usingOverrideColorScheme(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_usingOverrideColorScheme", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_usingOverrideEnvironment(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_usingOverrideEnvironment", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType for StandardLevelScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
