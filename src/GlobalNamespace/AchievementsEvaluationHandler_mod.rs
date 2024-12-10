#[cfg(feature = "AchievementsEvaluationHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct AchievementsEvaluationHandler {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _clearedLevel100Achievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _totalScore100MillionAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _24HoursPlayedAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _kilometersTravelled100Achievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _15ExpertLevelsRankSAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _15ExpertLevelsFullComboAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _15HardLevelsRankSAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _15HardLevelsFullComboAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _expertLevelClearedWithoutModifiersAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _fullComboExpertWithoutModifiersAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _goodCuts10000Achievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _resultMinRankANormalWithoutModifiersAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _resultMinRankSHardWithoutModifiersAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _resultMinRankSSExpertWithoutModifiersAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _combo50NormalWithoutModifiersAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _combo100HardWithoutModifiersAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _combo500ExpertWithoutModifiersAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _clearedLevelWithoutModifiersAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _clearedLevelWithSongSpeedFasterModifierAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _clearedLevelWithInstaFailModifierAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _clearedLevelWithDisappearingArrowsModifierAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _clearedLevelWithBatteryEnergyModifierAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _cleared30MissionsAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _finalMissionClearedAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _allMissionClearedAchievement: *mut crate::GlobalNamespace::AchievementSO,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _missionNodesManager: *mut crate::GlobalNamespace::IMissionNodesManager,
    pub _achievementsModel: *mut crate::GlobalNamespace::AchievementsModelSO,
}
#[cfg(feature = "AchievementsEvaluationHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AchievementsEvaluationHandler
    => ""."AchievementsEvaluationHandler"
);
#[cfg(feature = "AchievementsEvaluationHandler")]
impl std::ops::Deref for crate::GlobalNamespace::AchievementsEvaluationHandler {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AchievementsEvaluationHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::AchievementsEvaluationHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AchievementsEvaluationHandler")]
impl crate::GlobalNamespace::AchievementsEvaluationHandler {
    pub fn HandleCampaignOverallStatsDataDidUpdate(
        &mut self,
        missionCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionCompletionResults,
        >,
        missionNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMissionNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCampaignOverallStatsDataDidUpdate",
                (missionCompletionResults, missionNode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePartyFreePlayOverallStatsDataDidUpdate(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandlePartyFreePlayOverallStatsDataDidUpdate",
                (levelCompletionResults, beatmapDifficulty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSoloFreePlayOverallStatsDataDidUpdate(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSoloFreePlayOverallStatsDataDidUpdate",
                (levelCompletionResults, beatmapDifficulty),
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
    pub fn ProcessLevelFinishData(
        &mut self,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessLevelFinishData",
                (beatmapDifficulty, levelCompletionResults),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMissionFinishData(
        &mut self,
        missionNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMissionNode>,
        missionCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessMissionFinishData",
                (missionNode, missionCompletionResults),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessSoloFreePlayLevelFinishData(
        &mut self,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessSoloFreePlayLevelFinishData",
                (beatmapDifficulty, levelCompletionResults),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "AchievementsEvaluationHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AchievementsEvaluationHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
