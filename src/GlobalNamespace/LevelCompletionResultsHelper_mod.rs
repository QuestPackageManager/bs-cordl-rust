#[cfg(feature = "cordl_class_LevelCompletionResultsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelCompletionResultsHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_LevelCompletionResultsHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LevelCompletionResultsHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LevelCompletionResultsHelper";
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
#[cfg(feature = "cordl_class_LevelCompletionResultsHelper")]
impl std::ops::Deref for crate::GlobalNamespace::LevelCompletionResultsHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_LevelCompletionResultsHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelCompletionResultsHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IReadonlyBeatmapData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapObjectExecutionRating,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiersModelSO,
                            >,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<f32>,
                            >,
                            f32,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<f32>,
                            >,
                            f32,
                            f32,
                            crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType,
                            crate::GlobalNamespace::LevelCompletionResults_LevelEndAction,
                            f32,
                            f32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCompletionResults,
                        >,
                        18usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            18usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
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
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::BeatmapKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerLevelStatsData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCompletionResults,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IReadonlyBeatmapData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlatformLeaderboardsModel,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("ProcessScore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessScore", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        beatmapKey,
                        playerData,
                        playerLevelStats,
                        levelCompletionResults,
                        transformedBeatmapData,
                        platformLeaderboardsModel,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_LevelCompletionResultsHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelCompletionResultsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
