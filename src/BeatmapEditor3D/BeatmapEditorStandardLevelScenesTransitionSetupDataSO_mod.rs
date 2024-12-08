#[cfg(feature = "BeatmapEditor3D+BeatmapEditorStandardLevelScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEditorStandardLevelScenesTransitionSetupDataSO {
    __cordl_parent: LevelScenesTransitionSetupDataSO,
    pub _standardGameplaySceneInfo: *mut SceneInfo,
    pub _beatmapEditorGameplaySceneInfo: *mut SceneInfo,
    pub _gameCoreSceneInfo: *mut SceneInfo,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
        *mut LevelCompletionResults,
    >,
}
#[cfg(feature = "BeatmapEditor3D+BeatmapEditorStandardLevelScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO =>
    "BeatmapEditor3D"."BeatmapEditorStandardLevelScenesTransitionSetupDataSO"
);
#[cfg(feature = "BeatmapEditor3D+BeatmapEditorStandardLevelScenesTransitionSetupDataSO")]
impl std::ops::Deref
for crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO {
    type Target = LevelScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEditor3D+BeatmapEditorStandardLevelScenesTransitionSetupDataSO")]
impl std::ops::DerefMut
for crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEditor3D+BeatmapEditorStandardLevelScenesTransitionSetupDataSO")]
impl crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO {
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
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
            *mut crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
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
    pub fn Init(
        &mut self,
        beatmapLevelData: *mut IBeatmapLevelData,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
        practiceSettings: *mut PracticeSettings,
        colorScheme: *mut ColorScheme,
        environmentsListModel: *mut EnvironmentsListModel,
        audioClipAsyncLoader: *mut AudioClipAsyncLoader,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        useFirstPersonFlyingController: bool,
        beforeSceneSwitchCallback: *mut crate::System::Action,
        levelFinishedCallback: *mut crate::System::Action_2<
            *mut crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO,
            *mut LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    beatmapLevelData,
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    colorScheme,
                    environmentsListModel,
                    audioClipAsyncLoader,
                    performancePreset,
                    useFirstPersonFlyingController,
                    beforeSceneSwitchCallback,
                    levelFinishedCallback,
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
#[cfg(feature = "BeatmapEditor3D+BeatmapEditorStandardLevelScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapEditor3D::BeatmapEditorStandardLevelScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
