#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelScenesTransitionSetupDataSO {
    __cordl_parent: LevelScenesTransitionSetupDataSO,
    pub _missionGameplaySceneInfo: *mut SceneInfo,
    pub _gameCoreSceneInfo: *mut SceneInfo,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut MissionLevelScenesTransitionSetupDataSO,
        *mut MissionCompletionResults,
    >,
    pub _missionId_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionLevelScenesTransitionSetupDataSO => ""
    ."MissionLevelScenesTransitionSetupDataSO"
);
#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
impl std::ops::Deref for MissionLevelScenesTransitionSetupDataSO {
    type Target = LevelScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for MissionLevelScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
impl MissionLevelScenesTransitionSetupDataSO {
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MissionLevelScenesTransitionSetupDataSO,
            *mut MissionCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_missionId(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_missionId", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MissionLevelScenesTransitionSetupDataSO,
            *mut MissionCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Finish(
        &mut self,
        levelCompletionResults: *mut MissionCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", (levelCompletionResults))?;
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
    pub fn get_missionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_missionId", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init_IBeatmapLevelData_ByRefMut_BeatmapLevel_Il2CppArray_ColorScheme_GameplayModifiers_PlayerSpecificSettings_EnvironmentsListModel0(
        &mut self,
        missionId: *mut crate::System::String,
        beatmapLevelData: *mut IBeatmapLevelData,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut MissionObjective,
        >,
        overrideColorScheme: *mut ColorScheme,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
        environmentsListModel: *mut EnvironmentsListModel,
        audioClipAsyncLoader: *mut AudioClipAsyncLoader,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        beatmapDataLoader: *mut BeatmapDataLoader,
        backButtonText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    missionId,
                    beatmapLevelData,
                    beatmapKey,
                    beatmapLevel,
                    missionObjectives,
                    overrideColorScheme,
                    gameplayModifiers,
                    playerSpecificSettings,
                    environmentsListModel,
                    audioClipAsyncLoader,
                    performancePreset,
                    beatmapDataLoader,
                    backButtonText,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init_ByRefMut_BeatmapLevel_Il2CppArray_ColorScheme_GameplayModifiers_PlayerSpecificSettings_EnvironmentsListModel_BeatmapLevelsModel1(
        &mut self,
        missionId: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut MissionObjective,
        >,
        overrideColorScheme: *mut ColorScheme,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
        environmentsListModel: *mut EnvironmentsListModel,
        beatmapLevelsModel: *mut BeatmapLevelsModel,
        audioClipAsyncLoader: *mut AudioClipAsyncLoader,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        beatmapDataLoader: *mut BeatmapDataLoader,
        backButtonText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    missionId,
                    beatmapKey,
                    beatmapLevel,
                    missionObjectives,
                    overrideColorScheme,
                    gameplayModifiers,
                    playerSpecificSettings,
                    environmentsListModel,
                    beatmapLevelsModel,
                    audioClipAsyncLoader,
                    performancePreset,
                    beatmapDataLoader,
                    backButtonText,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType for MissionLevelScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
