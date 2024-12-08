#[cfg(feature = "GameplayCoreSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayCoreSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub beatmapBasicData: *mut crate::GlobalNamespace::BeatmapBasicData,
    pub beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
    pub gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    pub playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
    pub practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
    pub useTestNoteCutSoundEffects: bool,
    pub environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
    pub colorScheme: *mut crate::GlobalNamespace::ColorScheme,
    pub recordingToolData: crate::System::Nullable_1<
        crate::GlobalNamespace::RecordingToolManager_SetupData,
    >,
    pub _songAudioClip_k__BackingField: *mut crate::UnityEngine::AudioClip,
    pub _beatmapLevelData_k__BackingField: *mut crate::GlobalNamespace::IBeatmapLevelData,
    pub _transformedBeatmapData_k__BackingField: *mut crate::GlobalNamespace::IReadonlyBeatmapData,
    pub _performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
    pub _beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub _beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
    pub _audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
    pub _beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
    pub _allowNullBeatmapLevelData: bool,
    pub _enableBeatmapDataCaching: bool,
}
#[cfg(feature = "GameplayCoreSceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayCoreSceneSetupData =>
    ""."GameplayCoreSceneSetupData"
);
#[cfg(feature = "GameplayCoreSceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayCoreSceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayCoreSceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayCoreSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayCoreSceneSetupData")]
impl crate::GlobalNamespace::GameplayCoreSceneSetupData {
    #[cfg(feature = "GameplayCoreSceneSetupData+_LoadBeatmapLevelData_d__37")]
    pub type _LoadBeatmapLevelData_d__37 = crate::GlobalNamespace::GameplayCoreSceneSetupData__LoadBeatmapLevelData_d__37;
    #[cfg(feature = "GameplayCoreSceneSetupData+_LoadTransformedBeatmapDataAsync_d__35")]
    pub type _LoadTransformedBeatmapDataAsync_d__35 = crate::GlobalNamespace::GameplayCoreSceneSetupData__LoadTransformedBeatmapDataAsync_d__35;
    pub fn ApplyDisableUpdateAlwaysConfiguration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyDisableUpdateAlwaysConfiguration", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadBeatmapLevelData(
        &mut self,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::IBeatmapLevelData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::IBeatmapLevelData,
        > = __cordl_object.invoke("LoadBeatmapLevelData", (beatmapLevelDataVersion))?;
        Ok(__cordl_ret)
    }
    pub fn LoadTransformedBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadTransformedBeatmapData", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadTransformedBeatmapDataAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("LoadTransformedBeatmapDataAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_PerformancePreset_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool_Nullable_1_2(
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        useTestNoteCutSoundEffects: bool,
        environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
        beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
        beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        enableBeatmapDataCaching: bool,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    environmentInfo,
                    colorScheme,
                    performancePreset,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_PerformancePreset_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool__cordl_bool_Nullable_1_3(
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        useTestNoteCutSoundEffects: bool,
        environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
        beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
        beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        enableBeatmapDataCaching: bool,
        allowNullBeatmapLevelData: bool,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    environmentInfo,
                    colorScheme,
                    performancePreset,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    allowNullBeatmapLevelData,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_PerformancePreset_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsModel_BeatmapLevelsEntitlementModel__cordl_bool_Nullable_1_0(
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        useTestNoteCutSoundEffects: bool,
        environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
        beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
        beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
        beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        enableBeatmapDataCaching: bool,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    environmentInfo,
                    colorScheme,
                    performancePreset,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsModel,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_IBeatmapLevelData_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_PerformancePreset_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool_Nullable_1_1(
        beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        useTestNoteCutSoundEffects: bool,
        environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
        beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
        beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        enableBeatmapDataCaching: bool,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapLevelData,
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    environmentInfo,
                    colorScheme,
                    performancePreset,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn TransformBeatmapData(
        &mut self,
        beatmapData: *mut crate::GlobalNamespace::IReadonlyBeatmapData,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::IReadonlyBeatmapData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IReadonlyBeatmapData = __cordl_object
            .invoke("TransformBeatmapData", (beatmapData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_PerformancePreset_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool_Nullable_1_2(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        useTestNoteCutSoundEffects: bool,
        environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
        beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
        beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        enableBeatmapDataCaching: bool,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    environmentInfo,
                    colorScheme,
                    performancePreset,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_PerformancePreset_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool__cordl_bool_Nullable_1_3(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        useTestNoteCutSoundEffects: bool,
        environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
        beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
        beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        enableBeatmapDataCaching: bool,
        allowNullBeatmapLevelData: bool,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    environmentInfo,
                    colorScheme,
                    performancePreset,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    allowNullBeatmapLevelData,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_PerformancePreset_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsModel_BeatmapLevelsEntitlementModel__cordl_bool_Nullable_1_0(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        useTestNoteCutSoundEffects: bool,
        environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
        beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
        beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
        beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        enableBeatmapDataCaching: bool,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    environmentInfo,
                    colorScheme,
                    performancePreset,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsModel,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IBeatmapLevelData_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_PerformancePreset_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool_Nullable_1_1(
        &mut self,
        beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
        useTestNoteCutSoundEffects: bool,
        environmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
        beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
        beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        enableBeatmapDataCaching: bool,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapLevelData,
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    environmentInfo,
                    colorScheme,
                    performancePreset,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapLevelData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::IBeatmapLevelData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IBeatmapLevelData = __cordl_object
            .invoke("get_beatmapLevelData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songAudioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AudioClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AudioClip = __cordl_object
            .invoke("get_songAudioClip", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_transformedBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::IReadonlyBeatmapData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IReadonlyBeatmapData = __cordl_object
            .invoke("get_transformedBeatmapData", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapLevelData(
        &mut self,
        value: *mut crate::GlobalNamespace::IBeatmapLevelData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapLevelData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_songAudioClip(
        &mut self,
        value: *mut crate::UnityEngine::AudioClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_songAudioClip", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_transformedBeatmapData(
        &mut self,
        value: *mut crate::GlobalNamespace::IReadonlyBeatmapData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_transformedBeatmapData", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameplayCoreSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayCoreSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
