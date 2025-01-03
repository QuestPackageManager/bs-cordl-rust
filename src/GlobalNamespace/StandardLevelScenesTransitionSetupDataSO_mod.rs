#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelScenesTransitionSetupDataSO {
    __cordl_parent: crate::GlobalNamespace::LevelScenesTransitionSetupDataSO,
    pub _standardGameplaySceneInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SceneInfo,
    >,
    pub _gameCoreSceneInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::LevelCompletionResults,
        >,
    >,
    pub _gameMode_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _beatmapKey_k__BackingField: crate::GlobalNamespace::BeatmapKey,
    pub _beatmapLevel_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevel,
    >,
    pub _practiceSettings_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PracticeSettings,
    >,
    pub _usingOverrideColorScheme_k__BackingField: bool,
    pub _colorScheme_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorScheme,
    >,
    pub _usingOverrideEnvironment_k__BackingField: bool,
    pub _targetEnvironmentInfo_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentInfoSO,
    >,
    pub _originalEnvironmentInfo_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentInfoSO,
    >,
    pub _gameplayModifiers_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiers,
    >,
}
#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO => ""
    ."StandardLevelScenesTransitionSetupDataSO"
);
#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
impl std::ops::Deref
for crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO {
    type Target = crate::GlobalNamespace::LevelScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
impl std::ops::DerefMut
for crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
impl crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO {
    pub fn Finish(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", (levelCompletionResults))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAndSetupScenes(
        &mut self,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InitColorInfo(
        &mut self,
        playerOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        playerOverrideLightshowColors: bool,
        beatmapOverrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitColorInfo",
                (
                    playerOverrideColorScheme,
                    playerOverrideLightshowColors,
                    beatmapOverrideColorScheme,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitEnvironmentInfo(
        &mut self,
        overrideEnvironmentSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitEnvironmentInfo",
                (overrideEnvironmentSettings, environmentsListModel),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_ByRefMut_BeatmapLevel_OverrideEnvironmentSettings_ColorScheme__cordl_bool_ColorScheme_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_EnvironmentsListModel_AudioClipAsyncLoader_BeatmapDataLoader_Il2CppString_BeatmapLevelsModel_BeatmapLevelsEntitlementModel1(
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
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
        beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
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
                    playerOverrideColorScheme,
                    playerOverrideLightshowColors,
                    beatmapOverrideColorScheme,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    environmentsListModel,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    settingsManager,
                    backButtonText,
                    beatmapLevelsModel,
                    beatmapLevelsEntitlementModel,
                    useTestNoteCutSoundEffects,
                    startPaused,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_IBeatmapLevelData_ByRefMut_BeatmapLevel_OverrideEnvironmentSettings_ColorScheme__cordl_bool_ColorScheme_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_EnvironmentsListModel_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel_Il2CppString0(
        &mut self,
        gameMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
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
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                    playerOverrideColorScheme,
                    playerOverrideLightshowColors,
                    beatmapOverrideColorScheme,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    environmentsListModel,
                    audioClipAsyncLoader,
                    settingsManager,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    backButtonText,
                    useTestNoteCutSoundEffects,
                    startPaused,
                    recordingToolData,
                ),
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::LevelCompletionResults,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = __cordl_object
            .invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = __cordl_object.invoke("get_beatmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        > = __cordl_object.invoke("get_colorScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gameMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_gameMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = __cordl_object.invoke("get_gameplayModifiers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_originalEnvironmentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        > = __cordl_object.invoke("get_originalEnvironmentInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_practiceSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        > = __cordl_object.invoke("get_practiceSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetEnvironmentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        > = __cordl_object.invoke("get_targetEnvironmentInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_usingOverrideColorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_usingOverrideColorScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_usingOverrideEnvironment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_usingOverrideEnvironment", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                *mut crate::GlobalNamespace::LevelCompletionResults,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapKey(
        &mut self,
        value: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapKey", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapLevel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colorScheme(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorScheme", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gameMode(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gameplayModifiers(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameplayModifiers", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_originalEnvironmentInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_originalEnvironmentInfo", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_practiceSettings(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_practiceSettings", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetEnvironmentInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetEnvironmentInfo", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardLevelScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
