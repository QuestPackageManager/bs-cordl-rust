#[cfg(feature = "PrepareLevelCompletionResults")]
#[repr(C)]
#[derive(Debug)]
pub struct PrepareLevelCompletionResults {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _gameplayModifiersModelSO: *mut GameplayModifiersModelSO,
    pub _saberActivityCounter: *mut SaberActivityCounter,
    pub _beatmapObjectExecutionRatingsRecorder: *mut BeatmapObjectExecutionRatingsRecorder,
    pub _scoreController: *mut IScoreController,
    pub _gameEnergyCounter: *mut GameEnergyCounter,
    pub _beatmapData: *mut IReadonlyBeatmapData,
    pub _audioTimeSyncController: *mut AudioTimeSyncController,
    pub _gameplayModifiers: *mut GameplayModifiers,
    pub _comboController: *mut ComboController,
}
#[cfg(feature = "PrepareLevelCompletionResults")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PrepareLevelCompletionResults => ""
    ."PrepareLevelCompletionResults"
);
#[cfg(feature = "PrepareLevelCompletionResults")]
impl std::ops::Deref for PrepareLevelCompletionResults {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PrepareLevelCompletionResults")]
impl std::ops::DerefMut for PrepareLevelCompletionResults {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PrepareLevelCompletionResults")]
impl PrepareLevelCompletionResults {
    pub fn FillLevelCompletionResults(
        &mut self,
        levelEndStateType: crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType,
        levelEndAction: crate::GlobalNamespace::LevelCompletionResults_LevelEndAction,
    ) -> quest_hook::libil2cpp::Result<*mut LevelCompletionResults> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LevelCompletionResults = __cordl_object
            .invoke("FillLevelCompletionResults", (levelEndStateType, levelEndAction))?;
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
}
#[cfg(feature = "PrepareLevelCompletionResults")]
impl quest_hook::libil2cpp::ObjectType for PrepareLevelCompletionResults {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
