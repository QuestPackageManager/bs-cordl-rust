#[cfg(feature = "AchievementsEvaluationHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct AchievementsEvaluationHandler {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _clearedLevel100Achievement: *mut AchievementSO,
    pub _totalScore100MillionAchievement: *mut AchievementSO,
    pub _24HoursPlayedAchievement: *mut AchievementSO,
    pub _kilometersTravelled100Achievement: *mut AchievementSO,
    pub _15ExpertLevelsRankSAchievement: *mut AchievementSO,
    pub _15ExpertLevelsFullComboAchievement: *mut AchievementSO,
    pub _15HardLevelsRankSAchievement: *mut AchievementSO,
    pub _15HardLevelsFullComboAchievement: *mut AchievementSO,
    pub _expertLevelClearedWithoutModifiersAchievement: *mut AchievementSO,
    pub _fullComboExpertWithoutModifiersAchievement: *mut AchievementSO,
    pub _goodCuts10000Achievement: *mut AchievementSO,
    pub _resultMinRankANormalWithoutModifiersAchievement: *mut AchievementSO,
    pub _resultMinRankSHardWithoutModifiersAchievement: *mut AchievementSO,
    pub _resultMinRankSSExpertWithoutModifiersAchievement: *mut AchievementSO,
    pub _combo50NormalWithoutModifiersAchievement: *mut AchievementSO,
    pub _combo100HardWithoutModifiersAchievement: *mut AchievementSO,
    pub _combo500ExpertWithoutModifiersAchievement: *mut AchievementSO,
    pub _clearedLevelWithoutModifiersAchievement: *mut AchievementSO,
    pub _clearedLevelWithSongSpeedFasterModifierAchievement: *mut AchievementSO,
    pub _clearedLevelWithInstaFailModifierAchievement: *mut AchievementSO,
    pub _clearedLevelWithDisappearingArrowsModifierAchievement: *mut AchievementSO,
    pub _clearedLevelWithBatteryEnergyModifierAchievement: *mut AchievementSO,
    pub _cleared30MissionsAchievement: *mut AchievementSO,
    pub _finalMissionClearedAchievement: *mut AchievementSO,
    pub _allMissionClearedAchievement: *mut AchievementSO,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _missionNodesManager: *mut IMissionNodesManager,
    pub _achievementsModel: *mut AchievementsModelSO,
}
#[cfg(feature = "AchievementsEvaluationHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AchievementsEvaluationHandler => ""
    ."AchievementsEvaluationHandler"
);
#[cfg(feature = "AchievementsEvaluationHandler")]
impl std::ops::Deref for AchievementsEvaluationHandler {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AchievementsEvaluationHandler")]
impl std::ops::DerefMut for AchievementsEvaluationHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AchievementsEvaluationHandler")]
impl AchievementsEvaluationHandler {
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
    pub fn HandlePartyFreePlayOverallStatsDataDidUpdate(
        &mut self,
        levelCompletionResults: *mut LevelCompletionResults,
        beatmapDifficulty: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandlePartyFreePlayOverallStatsDataDidUpdate",
                (levelCompletionResults, beatmapDifficulty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessLevelFinishData(
        &mut self,
        beatmapDifficulty: BeatmapDifficulty,
        levelCompletionResults: *mut LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessLevelFinishData",
                (beatmapDifficulty, levelCompletionResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessSoloFreePlayLevelFinishData(
        &mut self,
        beatmapDifficulty: BeatmapDifficulty,
        levelCompletionResults: *mut LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessSoloFreePlayLevelFinishData",
                (beatmapDifficulty, levelCompletionResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMissionFinishData(
        &mut self,
        missionNode: *mut IMissionNode,
        missionCompletionResults: *mut MissionCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessMissionFinishData",
                (missionNode, missionCompletionResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSoloFreePlayOverallStatsDataDidUpdate(
        &mut self,
        levelCompletionResults: *mut LevelCompletionResults,
        beatmapDifficulty: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSoloFreePlayOverallStatsDataDidUpdate",
                (levelCompletionResults, beatmapDifficulty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleCampaignOverallStatsDataDidUpdate(
        &mut self,
        missionCompletionResults: *mut MissionCompletionResults,
        missionNode: *mut IMissionNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCampaignOverallStatsDataDidUpdate",
                (missionCompletionResults, missionNode),
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
#[cfg(feature = "AchievementsEvaluationHandler")]
impl quest_hook::libil2cpp::ObjectType for AchievementsEvaluationHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
