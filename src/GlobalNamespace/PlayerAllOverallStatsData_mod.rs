#[cfg(feature = "PlayerAllOverallStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerAllOverallStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _campaignOverallStatsData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    >,
    pub _soloFreePlayOverallStatsData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    >,
    pub _partyFreePlayOverallStatsData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    >,
    pub _onlinePlayOverallStatsData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    >,
    pub didUpdateSoloFreePlayOverallStatsDataEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
    >,
    pub didUpdatePartyFreePlayOverallStatsDataEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
    >,
    pub didUpdateCampaignOverallStatsDataEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionCompletionResults>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMissionNode>,
        >,
    >,
}
#[cfg(feature = "PlayerAllOverallStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerAllOverallStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerAllOverallStatsData";
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
#[cfg(feature = "PlayerAllOverallStatsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerAllOverallStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerAllOverallStatsData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerAllOverallStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerAllOverallStatsData")]
impl crate::GlobalNamespace::PlayerAllOverallStatsData {
    #[cfg(feature = "PlayerAllOverallStatsData+PlayerOverallStatsData")]
    pub type PlayerOverallStatsData = crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData;
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_PlayerAllOverallStatsData_PlayerOverallStatsData_PlayerAllOverallStatsData_PlayerOverallStatsData_PlayerAllOverallStatsData_PlayerOverallStatsData_PlayerAllOverallStatsData_PlayerOverallStatsData1(
        campaignOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
        soloFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
        partyFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
        onlinePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    campaignOverallStatsData,
                    soloFreePlayOverallStatsData,
                    partyFreePlayOverallStatsData,
                    onlinePlayOverallStatsData,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateCampaignOverallStatsData(
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
                "UpdateCampaignOverallStatsData",
                (missionCompletionResults, missionNode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateOnlinePlayOverallStatsData(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateOnlinePlayOverallStatsData", (levelCompletionResults))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePartyFreePlayOverallStatsData(
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
                "UpdatePartyFreePlayOverallStatsData",
                (levelCompletionResults, beatmapDifficulty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSoloFreePlayOverallStatsData(
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
                "UpdateSoloFreePlayOverallStatsData",
                (levelCompletionResults, beatmapDifficulty),
            )?;
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
    pub fn _ctor_PlayerAllOverallStatsData_PlayerOverallStatsData_PlayerAllOverallStatsData_PlayerOverallStatsData_PlayerAllOverallStatsData_PlayerOverallStatsData_PlayerAllOverallStatsData_PlayerOverallStatsData1(
        &mut self,
        campaignOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
        soloFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
        partyFreePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
        onlinePlayOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    campaignOverallStatsData,
                    soloFreePlayOverallStatsData,
                    partyFreePlayOverallStatsData,
                    onlinePlayOverallStatsData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didUpdateCampaignOverallStatsDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionCompletionResults,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMissionNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didUpdateCampaignOverallStatsDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didUpdatePartyFreePlayOverallStatsDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCompletionResults,
                >,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didUpdatePartyFreePlayOverallStatsDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didUpdateSoloFreePlayOverallStatsDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCompletionResults,
                >,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didUpdateSoloFreePlayOverallStatsDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allOverallStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = __cordl_object.invoke("get_allOverallStatsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_campaignOverallStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = __cordl_object.invoke("get_campaignOverallStatsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onlinePlayOverallStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = __cordl_object.invoke("get_onlinePlayOverallStatsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_partyFreePlayOverallStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = __cordl_object.invoke("get_partyFreePlayOverallStatsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_soloFreePlayOverallStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = __cordl_object.invoke("get_soloFreePlayOverallStatsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didUpdateCampaignOverallStatsDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionCompletionResults,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMissionNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didUpdateCampaignOverallStatsDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didUpdatePartyFreePlayOverallStatsDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCompletionResults,
                >,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didUpdatePartyFreePlayOverallStatsDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didUpdateSoloFreePlayOverallStatsDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCompletionResults,
                >,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didUpdateSoloFreePlayOverallStatsDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerAllOverallStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerAllOverallStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerAllOverallStatsData+PlayerOverallStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerAllOverallStatsData_PlayerOverallStatsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _goodCutsCount_k__BackingField: i32,
    pub _badCutsCount_k__BackingField: i32,
    pub _missedCutsCount_k__BackingField: i32,
    pub _totalScore_k__BackingField: i64,
    pub _playedLevelsCount_k__BackingField: i32,
    pub _clearedLevelsCount_k__BackingField: i32,
    pub _failedLevelsCount_k__BackingField: i32,
    pub _fullComboCount_k__BackingField: i32,
    pub _timePlayed_k__BackingField: f32,
    pub _handDistanceTravelled_k__BackingField: i32,
    pub _totalCutScore_k__BackingField: i64,
}
#[cfg(feature = "PlayerAllOverallStatsData+PlayerOverallStatsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerOverallStatsData";
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
#[cfg(feature = "PlayerAllOverallStatsData+PlayerOverallStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerAllOverallStatsData+PlayerOverallStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerAllOverallStatsData+PlayerOverallStatsData")]
impl crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_i32_i64_i32_i32_i32_i32_f32_i32_i64_1(
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCutsCount: i32,
        totalScore: i64,
        playedLevelsCount: i32,
        clearedLevelsCount: i32,
        failedLevelsCount: i32,
        fullComboCount: i32,
        timePlayed: f32,
        handDistanceTravelled: i32,
        totalCutScore: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    goodCutsCount,
                    badCutsCount,
                    missedCutsCount,
                    totalScore,
                    playedLevelsCount,
                    clearedLevelsCount,
                    failedLevelsCount,
                    fullComboCount,
                    timePlayed,
                    handDistanceTravelled,
                    totalCutScore,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateWithLevelCompletionResults(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateWithLevelCompletionResults", (levelCompletionResults))?;
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
    pub fn _ctor_i32_i32_i32_i64_i32_i32_i32_i32_f32_i32_i64_1(
        &mut self,
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCutsCount: i32,
        totalScore: i64,
        playedLevelsCount: i32,
        clearedLevelsCount: i32,
        failedLevelsCount: i32,
        fullComboCount: i32,
        timePlayed: f32,
        handDistanceTravelled: i32,
        totalCutScore: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    goodCutsCount,
                    badCutsCount,
                    missedCutsCount,
                    totalScore,
                    playedLevelsCount,
                    clearedLevelsCount,
                    failedLevelsCount,
                    fullComboCount,
                    timePlayed,
                    handDistanceTravelled,
                    totalCutScore,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_badCutsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_badCutsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clearedLevelsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_clearedLevelsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_failedLevelsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_failedLevelsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fullComboCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_fullComboCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_goodCutsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_goodCutsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_handDistanceTravelled(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_handDistanceTravelled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_missedCutsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_missedCutsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playedLevelsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_playedLevelsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timePlayed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timePlayed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalCutScore(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_totalCutScore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalScore(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_totalScore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        a: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_badCutsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_badCutsCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clearedLevelsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clearedLevelsCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_failedLevelsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_failedLevelsCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fullComboCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fullComboCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_goodCutsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_goodCutsCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_handDistanceTravelled(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_handDistanceTravelled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_missedCutsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_missedCutsCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_playedLevelsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playedLevelsCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_timePlayed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_timePlayed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_totalCutScore(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_totalCutScore", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_totalScore(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_totalScore", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerAllOverallStatsData+PlayerOverallStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
