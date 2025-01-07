#[cfg(feature = "PlayerLevelStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerLevelStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _highScore: i32,
    pub _maxCombo: i32,
    pub _fullCombo: bool,
    pub _maxRank: crate::GlobalNamespace::RankModel_Rank,
    pub _validScore: bool,
    pub _playCount: i32,
    pub _levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub _beatmapCharacteristic: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
}
#[cfg(feature = "PlayerLevelStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerLevelStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerLevelStatsData";
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
#[cfg(feature = "PlayerLevelStatsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerLevelStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerLevelStatsData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerLevelStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerLevelStatsData")]
impl crate::GlobalNamespace::PlayerLevelStatsData {
    pub fn GetBeatmapKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = __cordl_object
            .invoke("GetBeatmapKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IncreaseNumberOfGameplays(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncreaseNumberOfGameplays", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_ByRefMut0(
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapKey))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_BeatmapDifficulty_BeatmapCharacteristicSO1(
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelID, difficulty, beatmapCharacteristic))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_BeatmapDifficulty_BeatmapCharacteristicSO_i32_i32__cordl_bool_RankModel_Rank__cordl_bool_i32_2(
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        highScore: i32,
        maxCombo: i32,
        fullCombo: bool,
        maxRank: crate::GlobalNamespace::RankModel_Rank,
        validScore: bool,
        playCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    levelID,
                    difficulty,
                    beatmapCharacteristic,
                    highScore,
                    maxCombo,
                    fullCombo,
                    maxRank,
                    validScore,
                    playCount,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateScoreData(
        &mut self,
        score: i32,
        maxCombo: i32,
        fullCombo: bool,
        rank: crate::GlobalNamespace::RankModel_Rank,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScoreData", (score, maxCombo, fullCombo, rank))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut0(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_BeatmapDifficulty_BeatmapCharacteristicSO1(
        &mut self,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelID, difficulty, beatmapCharacteristic))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_BeatmapDifficulty_BeatmapCharacteristicSO_i32_i32__cordl_bool_RankModel_Rank__cordl_bool_i32_2(
        &mut self,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        highScore: i32,
        maxCombo: i32,
        fullCombo: bool,
        maxRank: crate::GlobalNamespace::RankModel_Rank,
        validScore: bool,
        playCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    levelID,
                    difficulty,
                    beatmapCharacteristic,
                    highScore,
                    maxCombo,
                    fullCombo,
                    maxRank,
                    validScore,
                    playCount,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapCharacteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        > = __cordl_object.invoke("get_beatmapCharacteristic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficulty = __cordl_object
            .invoke("get_difficulty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fullCombo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_fullCombo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_highScore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_levelID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxCombo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxCombo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxRank(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::RankModel_Rank> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::RankModel_Rank = __cordl_object
            .invoke("get_maxRank", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_playCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_validScore(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_validScore", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerLevelStatsData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlayerLevelStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
