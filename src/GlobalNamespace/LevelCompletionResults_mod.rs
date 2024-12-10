#[cfg(feature = "LevelCompletionResults")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelCompletionResults {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    pub modifiedScore: i32,
    pub multipliedScore: i32,
    pub rank: crate::GlobalNamespace::RankModel_Rank,
    pub fullCombo: bool,
    pub leftSaberMovementDistance: f32,
    pub rightSaberMovementDistance: f32,
    pub leftHandMovementDistance: f32,
    pub rightHandMovementDistance: f32,
    pub levelEndStateType: crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType,
    pub levelEndAction: crate::GlobalNamespace::LevelCompletionResults_LevelEndAction,
    pub energy: f32,
    pub goodCutsCount: i32,
    pub badCutsCount: i32,
    pub missedCount: i32,
    pub notGoodCount: i32,
    pub okCount: i32,
    pub maxCutScore: i32,
    pub totalCutScore: i32,
    pub goodCutsCountForNotesWithFullScoreScoringType: i32,
    pub averageCenterDistanceCutScoreForNotesWithFullScoreScoringType: f32,
    pub averageCutScoreForNotesWithFullScoreScoringType: f32,
    pub maxCombo: i32,
    pub endSongTime: f32,
    pub invalidated: bool,
}
#[cfg(feature = "LevelCompletionResults")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelCompletionResults => ""
    ."LevelCompletionResults"
);
#[cfg(feature = "LevelCompletionResults")]
impl std::ops::Deref for crate::GlobalNamespace::LevelCompletionResults {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResults")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelCompletionResults {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResults")]
impl crate::GlobalNamespace::LevelCompletionResults {
    #[cfg(feature = "LevelCompletionResults+LevelEndAction")]
    pub type LevelEndAction = crate::GlobalNamespace::LevelCompletionResults_LevelEndAction;
    #[cfg(feature = "LevelCompletionResults+LevelEndStateType")]
    pub type LevelEndStateType = crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType;
    pub fn CompareTo(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_Utils_INetImmutableSerializable_LevelCompletionResults__CreateFromSerializedData(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        > = __cordl_object
            .invoke(
                "LiteNetLib.Utils.INetImmutableSerializable<LevelCompletionResults>.CreateFromSerializedData",
                (reader),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_GameplayModifiers_i32_i32_RankModel_Rank__cordl_bool_f32_f32_f32_f32_LevelCompletionResults_LevelEndStateType_LevelCompletionResults_LevelEndAction_f32_i32_i32_i32_i32_i32_i32_i32_i32_f32_f32_i32_f32__cordl_bool1(
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        modifiedScore: i32,
        multipliedScore: i32,
        rank: crate::GlobalNamespace::RankModel_Rank,
        fullCombo: bool,
        leftSaberMovementDistance: f32,
        rightSaberMovementDistance: f32,
        leftHandMovementDistance: f32,
        rightHandMovementDistance: f32,
        levelEndStateType: crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType,
        levelEndAction: crate::GlobalNamespace::LevelCompletionResults_LevelEndAction,
        energy: f32,
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCount: i32,
        notGoodCount: i32,
        okCount: i32,
        maxCutScore: i32,
        totalCutScore: i32,
        goodCutsCountForNotesWithFullScoreScoringType: i32,
        averageCenterDistanceCutScoreForNotesWithFullScoreScoringType: f32,
        averageCutScoreForNotesWithFullScoreScoringType: f32,
        maxCombo: i32,
        endSongTime: f32,
        invalidated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    gameplayModifiers,
                    modifiedScore,
                    multipliedScore,
                    rank,
                    fullCombo,
                    leftSaberMovementDistance,
                    rightSaberMovementDistance,
                    leftHandMovementDistance,
                    rightHandMovementDistance,
                    levelEndStateType,
                    levelEndAction,
                    energy,
                    goodCutsCount,
                    badCutsCount,
                    missedCount,
                    notGoodCount,
                    okCount,
                    maxCutScore,
                    totalCutScore,
                    goodCutsCountForNotesWithFullScoreScoringType,
                    averageCenterDistanceCutScoreForNotesWithFullScoreScoringType,
                    averageCutScoreForNotesWithFullScoreScoringType,
                    maxCombo,
                    endSongTime,
                    invalidated,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GameplayModifiers_i32_i32_RankModel_Rank__cordl_bool_f32_f32_f32_f32_LevelCompletionResults_LevelEndStateType_LevelCompletionResults_LevelEndAction_f32_i32_i32_i32_i32_i32_i32_i32_i32_f32_f32_i32_f32__cordl_bool1(
        &mut self,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        modifiedScore: i32,
        multipliedScore: i32,
        rank: crate::GlobalNamespace::RankModel_Rank,
        fullCombo: bool,
        leftSaberMovementDistance: f32,
        rightSaberMovementDistance: f32,
        leftHandMovementDistance: f32,
        rightHandMovementDistance: f32,
        levelEndStateType: crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType,
        levelEndAction: crate::GlobalNamespace::LevelCompletionResults_LevelEndAction,
        energy: f32,
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCount: i32,
        notGoodCount: i32,
        okCount: i32,
        maxCutScore: i32,
        totalCutScore: i32,
        goodCutsCountForNotesWithFullScoreScoringType: i32,
        averageCenterDistanceCutScoreForNotesWithFullScoreScoringType: f32,
        averageCutScoreForNotesWithFullScoreScoringType: f32,
        maxCombo: i32,
        endSongTime: f32,
        invalidated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    gameplayModifiers,
                    modifiedScore,
                    multipliedScore,
                    rank,
                    fullCombo,
                    leftSaberMovementDistance,
                    rightSaberMovementDistance,
                    leftHandMovementDistance,
                    rightHandMovementDistance,
                    levelEndStateType,
                    levelEndAction,
                    energy,
                    goodCutsCount,
                    badCutsCount,
                    missedCount,
                    notGoodCount,
                    okCount,
                    maxCutScore,
                    totalCutScore,
                    goodCutsCountForNotesWithFullScoreScoringType,
                    averageCenterDistanceCutScoreForNotesWithFullScoreScoringType,
                    averageCutScoreForNotesWithFullScoreScoringType,
                    maxCombo,
                    endSongTime,
                    invalidated,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelCompletionResults")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelCompletionResults {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LevelCompletionResults+LevelEndAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelCompletionResults_LevelEndAction {
    None = 0i32,
    Quit = 1i32,
    Restart = 2i32,
}
#[cfg(feature = "LevelCompletionResults+LevelEndAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelCompletionResults_LevelEndAction => ""
    ."LevelCompletionResults/LevelEndAction"
);
#[cfg(feature = "LevelCompletionResults+LevelEndStateType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelCompletionResults_LevelEndStateType {
    Cleared = 1i32,
    Failed = 2i32,
    Incomplete = 0i32,
}
#[cfg(feature = "LevelCompletionResults+LevelEndStateType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelCompletionResults_LevelEndStateType => ""
    ."LevelCompletionResults/LevelEndStateType"
);
