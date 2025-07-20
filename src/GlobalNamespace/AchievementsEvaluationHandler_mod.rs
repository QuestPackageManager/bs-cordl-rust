#[cfg(feature = "AchievementsEvaluationHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct AchievementsEvaluationHandler {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _clearedLevel100Achievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _totalScore100MillionAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _24HoursPlayedAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _kilometersTravelled100Achievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _15ExpertLevelsRankSAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _15ExpertLevelsFullComboAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _15HardLevelsRankSAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _15HardLevelsFullComboAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _expertLevelClearedWithoutModifiersAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _fullComboExpertWithoutModifiersAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _goodCuts10000Achievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _resultMinRankANormalWithoutModifiersAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _resultMinRankSHardWithoutModifiersAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _resultMinRankSSExpertWithoutModifiersAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _combo50NormalWithoutModifiersAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _combo100HardWithoutModifiersAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _combo500ExpertWithoutModifiersAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _clearedLevelWithoutModifiersAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _clearedLevelWithSongSpeedFasterModifierAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _clearedLevelWithInstaFailModifierAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _clearedLevelWithDisappearingArrowsModifierAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _clearedLevelWithBatteryEnergyModifierAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _cleared30MissionsAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _finalMissionClearedAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _allMissionClearedAchievement: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementSO,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _missionNodesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMissionNodesManager,
    >,
    pub _achievementsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementsModelSO,
    >,
}
#[cfg(feature = "AchievementsEvaluationHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AchievementsEvaluationHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AchievementsEvaluationHandler";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "AchievementsEvaluationHandler")]
impl std::ops::Deref for crate::GlobalNamespace::AchievementsEvaluationHandler {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AchievementsEvaluationHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::AchievementsEvaluationHandler {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MissionCompletionResults,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IMissionNode,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleCampaignOverallStatsDataDidUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleCampaignOverallStatsDataDidUpdate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (missionCompletionResults, missionNode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandlePartyFreePlayOverallStatsDataDidUpdate(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCompletionResults,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandlePartyFreePlayOverallStatsDataDidUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandlePartyFreePlayOverallStatsDataDidUpdate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (levelCompletionResults, beatmapDifficulty))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSoloFreePlayOverallStatsDataDidUpdate(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCompletionResults,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleSoloFreePlayOverallStatsDataDidUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleSoloFreePlayOverallStatsDataDidUpdate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (levelCompletionResults, beatmapDifficulty))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnDestroy", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessLevelFinishData(
        &mut self,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::BeatmapDifficulty,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCompletionResults,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessLevelFinishData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessLevelFinishData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (beatmapDifficulty, levelCompletionResults))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMissionFinishData(
        &mut self,
        missionNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMissionNode>,
        missionCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IMissionNode,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MissionCompletionResults,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessMissionFinishData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessMissionFinishData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (missionNode, missionCompletionResults))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessSoloFreePlayLevelFinishData(
        &mut self,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::BeatmapDifficulty,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCompletionResults,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessSoloFreePlayLevelFinishData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessSoloFreePlayLevelFinishData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (beatmapDifficulty, levelCompletionResults))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
