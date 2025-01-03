#[cfg(feature = "GameplayCoreSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayCoreSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub beatmapBasicData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapBasicData,
    >,
    pub beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    pub gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiers,
    >,
    pub playerSpecificSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSpecificSettings,
    >,
    pub practiceSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PracticeSettings,
    >,
    pub useTestNoteCutSoundEffects: bool,
    pub targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentInfoSO,
    >,
    pub originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentInfoSO,
    >,
    pub colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    pub recordingToolData: crate::System::Nullable_1<
        crate::GlobalNamespace::RecordingToolManager_SetupData,
    >,
    pub environmentsListModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentsListModel,
    >,
    pub _songAudioClip_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AudioClip,
    >,
    pub _beatmapLevelData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatmapLevelData,
    >,
    pub _transformedBeatmapData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IReadonlyBeatmapData,
    >,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub _beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
    >,
    pub _audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioClipAsyncLoader,
    >,
    pub _beatmapDataLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataLoader,
    >,
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
    pub fn ApplyDisableUpdateAlwaysConfiguration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyDisableUpdateAlwaysConfiguration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapLevelData(
        &mut self,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::IBeatmapLevelData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::IBeatmapLevelData,
            >,
        > = __cordl_object.invoke("LoadBeatmapLevelData", (beatmapLevelDataVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadTransformedBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadTransformedBeatmapData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadTransformedBeatmapDataAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("LoadTransformedBeatmapDataAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_SettingsManager_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool_EnvironmentsListModel_Nullable_1_2(
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
        useTestNoteCutSoundEffects: bool,
        targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        enableBeatmapDataCaching: bool,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
                    targetEnvironmentInfo,
                    originalEnvironmentInfo,
                    colorScheme,
                    settingsManager,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    environmentsListModel,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_SettingsManager_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool__cordl_bool_EnvironmentsListModel_Nullable_1_3(
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
        useTestNoteCutSoundEffects: bool,
        targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        enableBeatmapDataCaching: bool,
        allowNullBeatmapLevelData: bool,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
                    targetEnvironmentInfo,
                    originalEnvironmentInfo,
                    colorScheme,
                    settingsManager,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    allowNullBeatmapLevelData,
                    environmentsListModel,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_SettingsManager_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsModel_BeatmapLevelsEntitlementModel__cordl_bool_EnvironmentsListModel_Nullable_1_0(
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
        useTestNoteCutSoundEffects: bool,
        targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
        beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        enableBeatmapDataCaching: bool,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
                    targetEnvironmentInfo,
                    originalEnvironmentInfo,
                    colorScheme,
                    settingsManager,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsModel,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    environmentsListModel,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_IBeatmapLevelData_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_SettingsManager_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool_EnvironmentsListModel_Nullable_1_1(
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
        useTestNoteCutSoundEffects: bool,
        targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        enableBeatmapDataCaching: bool,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        recordingToolData: crate::System::Nullable_1<
            crate::GlobalNamespace::RecordingToolManager_SetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
                    targetEnvironmentInfo,
                    originalEnvironmentInfo,
                    colorScheme,
                    settingsManager,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    environmentsListModel,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn TransformBeatmapData(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = __cordl_object.invoke("TransformBeatmapData", (beatmapData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_SettingsManager_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool_EnvironmentsListModel_Nullable_1_2(
        &mut self,
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
        useTestNoteCutSoundEffects: bool,
        targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        enableBeatmapDataCaching: bool,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
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
                ".ctor",
                (
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    targetEnvironmentInfo,
                    originalEnvironmentInfo,
                    colorScheme,
                    settingsManager,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    environmentsListModel,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_SettingsManager_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool__cordl_bool_EnvironmentsListModel_Nullable_1_3(
        &mut self,
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
        useTestNoteCutSoundEffects: bool,
        targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        enableBeatmapDataCaching: bool,
        allowNullBeatmapLevelData: bool,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
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
                ".ctor",
                (
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    targetEnvironmentInfo,
                    originalEnvironmentInfo,
                    colorScheme,
                    settingsManager,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    allowNullBeatmapLevelData,
                    environmentsListModel,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_SettingsManager_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsModel_BeatmapLevelsEntitlementModel__cordl_bool_EnvironmentsListModel_Nullable_1_0(
        &mut self,
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
        useTestNoteCutSoundEffects: bool,
        targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
        beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        enableBeatmapDataCaching: bool,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
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
                ".ctor",
                (
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    targetEnvironmentInfo,
                    originalEnvironmentInfo,
                    colorScheme,
                    settingsManager,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsModel,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    environmentsListModel,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IBeatmapLevelData_ByRefMut_BeatmapLevel_GameplayModifiers_PlayerSpecificSettings_PracticeSettings__cordl_bool_EnvironmentInfoSO_ColorScheme_SettingsManager_AudioClipAsyncLoader_BeatmapDataLoader_BeatmapLevelsEntitlementModel__cordl_bool_EnvironmentsListModel_Nullable_1_1(
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
        useTestNoteCutSoundEffects: bool,
        targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        beatmapLevelsEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        enableBeatmapDataCaching: bool,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
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
                ".ctor",
                (
                    beatmapLevelData,
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    useTestNoteCutSoundEffects,
                    targetEnvironmentInfo,
                    originalEnvironmentInfo,
                    colorScheme,
                    settingsManager,
                    audioClipAsyncLoader,
                    beatmapDataLoader,
                    beatmapLevelsEntitlementModel,
                    enableBeatmapDataCaching,
                    environmentsListModel,
                    recordingToolData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapLevelData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        > = __cordl_object.invoke("get_beatmapLevelData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songAudioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = __cordl_object
            .invoke("get_songAudioClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transformedBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = __cordl_object.invoke("get_transformedBeatmapData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapLevelData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapLevelData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_songAudioClip(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_songAudioClip", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_transformedBeatmapData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_transformedBeatmapData", (value))?;
        Ok(__cordl_ret.into())
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
