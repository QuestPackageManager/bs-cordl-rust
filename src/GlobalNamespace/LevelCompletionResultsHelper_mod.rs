#[cfg(feature = "LevelCompletionResultsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelCompletionResultsHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "LevelCompletionResultsHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelCompletionResultsHelper =>
    ""."LevelCompletionResultsHelper"
);
#[cfg(feature = "LevelCompletionResultsHelper")]
impl std::ops::Deref for crate::GlobalNamespace::LevelCompletionResultsHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResultsHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelCompletionResultsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResultsHelper")]
impl crate::GlobalNamespace::LevelCompletionResultsHelper {
    pub fn Create(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        beatmapObjectExecutionRatings: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapObjectExecutionRating,
                >,
            >,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        gameplayModifiersModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiersModelSO,
        >,
        multipliedScore: i32,
        modifiedScore: i32,
        maxCombo: i32,
        saberActivityValues: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        >,
        leftSaberMovementDistance: f32,
        rightSaberMovementDistance: f32,
        handActivityValues: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        >,
        leftHandMovementDistance: f32,
        rightHandMovementDistance: f32,
        levelEndStateType: crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType,
        levelEndAction: crate::GlobalNamespace::LevelCompletionResults_LevelEndAction,
        energy: f32,
        songTime: f32,
        invalidated: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (
                    beatmapData,
                    beatmapObjectExecutionRatings,
                    gameplayModifiers,
                    gameplayModifiersModel,
                    multipliedScore,
                    modifiedScore,
                    maxCombo,
                    saberActivityValues,
                    leftSaberMovementDistance,
                    rightSaberMovementDistance,
                    handActivityValues,
                    leftHandMovementDistance,
                    rightHandMovementDistance,
                    levelEndStateType,
                    levelEndAction,
                    energy,
                    songTime,
                    invalidated,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessScore(
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        playerData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
        playerLevelStats: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerLevelStatsData,
        >,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        transformedBeatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        platformLeaderboardsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ProcessScore",
                (
                    beatmapKey,
                    playerData,
                    playerLevelStats,
                    levelCompletionResults,
                    transformedBeatmapData,
                    platformLeaderboardsModel,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelCompletionResultsHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelCompletionResultsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
