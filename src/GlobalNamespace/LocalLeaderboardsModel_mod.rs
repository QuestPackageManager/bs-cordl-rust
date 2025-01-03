#[cfg(feature = "LocalLeaderboardsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalLeaderboardsModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub newScoreWasAddedToLeaderboardEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
        >,
    >,
    pub _fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
    pub _maxNumberOfScoresInLeaderboard: i32,
    pub _lastScorePositions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
            i32,
        >,
    >,
    pub _lastScoreLeaderboardId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _leaderboardsData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
        >,
    >,
    pub _dailyLeaderboardsData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
        >,
    >,
}
#[cfg(feature = "LocalLeaderboardsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LocalLeaderboardsModel => ""
    ."LocalLeaderboardsModel"
);
#[cfg(feature = "LocalLeaderboardsModel")]
impl std::ops::Deref for crate::GlobalNamespace::LocalLeaderboardsModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalLeaderboardsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardsModel")]
impl crate::GlobalNamespace::LocalLeaderboardsModel {
    pub const kLocalDailyLeaderboardsFileName: &'static str = "LocalDailyLeaderboards.dat";
    pub const kLocalLeaderboardsFileName: &'static str = "LocalLeaderboards.dat";
    #[cfg(feature = "LocalLeaderboardsModel+LeaderboardData")]
    pub type LeaderboardData = crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData;
    #[cfg(feature = "LocalLeaderboardsModel+LeaderboardType")]
    pub type LeaderboardType = crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType;
    #[cfg(feature = "LocalLeaderboardsModel+SavedLeaderboardsData")]
    pub type SavedLeaderboardsData = crate::GlobalNamespace::LocalLeaderboardsModel_SavedLeaderboardsData;
    #[cfg(feature = "LocalLeaderboardsModel+ScoreData")]
    pub type ScoreData = crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData;
    pub fn AddScore_Il2CppString_i32__cordl_bool1(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i32,
        fullCombo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddScore", (leaderboardId, playerName, score, fullCombo))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddScore_LocalLeaderboardsModel_LeaderboardType_Il2CppString_i32__cordl_bool0(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i32,
        fullCombo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddScore",
                (leaderboardId, leaderboardType, playerName, score, fullCombo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendLeaderboardScores(
        main: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData,
            >,
        >,
        tail: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData,
            >,
        >,
        maxNumberOfScores: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendLeaderboardScores", (main, tail, maxNumberOfScores))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearAllLeaderboardsAsync(
        &mut self,
        deleteLeaderboardFile: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ClearAllLeaderboardsAsync", (deleteLeaderboardFile))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearLastScorePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLastScorePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearLeaderboard(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLeaderboard", (leaderboardId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentTimestamp(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetCurrentTimestamp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHighScore(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetHighScore", (leaderboardId, leaderboardType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastScorePosition(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetLastScorePosition", (leaderboardId, leaderboardType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLeaderboardData(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
        > = __cordl_object
            .invoke("GetLeaderboardData", (leaderboardId, leaderboardType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLeaderboardsData(
        &mut self,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
            >,
        > = __cordl_object.invoke("GetLeaderboardsData", (leaderboardType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPositionInLeaderboard(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
        score: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "GetPositionInLeaderboard",
                (leaderboardId, leaderboardType, score),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetScores(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData,
            >,
        > = __cordl_object.invoke("GetScores", (leaderboardId, leaderboardType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Load", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("LoadAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLeaderboardsData(
        &mut self,
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
            >,
        > = __cordl_object.invoke("LoadLeaderboardsData", (filename))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLeaderboardsDataAsync(
        &mut self,
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Collections::Generic::List_1<
                    *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Collections::Generic::List_1<
                    *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
                >,
            >,
        > = __cordl_object.invoke("LoadLeaderboardsDataAsync", (filename))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLeaderboardsDataInternal(
        savedLeaderboardsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LocalLeaderboardsModel_SavedLeaderboardsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadLeaderboardsDataInternal", (savedLeaderboardsData))?;
        Ok(__cordl_ret.into())
    }
    pub fn MigrateQuestLeaderboards(
        leaderboardDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
            >,
        >,
        maxNumberOfScores: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MigrateQuestLeaderboards",
                (leaderboardDataList, maxNumberOfScores),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        localLeaderboardsSettingsSo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LocalLeaderboardsSettingsSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fileStorage, localLeaderboardsSettingsSo))?;
        Ok(__cordl_object.into())
    }
    pub fn SaveAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("SaveAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveLeaderboardsData(
        &mut self,
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("SaveLeaderboardsData", (filename, leaderboardsData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveLeaderboardsDataAsync(
        &mut self,
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("SaveLeaderboardsDataAsync", (filename, json))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDailyLeaderboard(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDailyLeaderboard", (leaderboardId))?;
        Ok(__cordl_ret.into())
    }
    pub fn WillScoreGoIntoLeaderboard_LocalLeaderboardsModel_LeaderboardType_i32_0(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
        score: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "WillScoreGoIntoLeaderboard",
                (leaderboardId, leaderboardType, score),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WillScoreGoIntoLeaderboard_i32_1(
        &mut self,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("WillScoreGoIntoLeaderboard", (leaderboardId, score))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        localLeaderboardsSettingsSo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LocalLeaderboardsSettingsSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fileStorage, localLeaderboardsSettingsSo))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_newScoreWasAddedToLeaderboardEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_newScoreWasAddedToLeaderboardEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_newScoreWasAddedToLeaderboardEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_newScoreWasAddedToLeaderboardEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LocalLeaderboardsModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalLeaderboardsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LocalLeaderboardsModel+LeaderboardData")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalLeaderboardsModel_LeaderboardData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _scores: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData,
        >,
    >,
}
#[cfg(feature = "LocalLeaderboardsModel+LeaderboardData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData => ""
    ."LocalLeaderboardsModel/LeaderboardData"
);
#[cfg(feature = "LocalLeaderboardsModel+LeaderboardData")]
impl std::ops::Deref for crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardsModel+LeaderboardData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardsModel+LeaderboardData")]
impl crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "LocalLeaderboardsModel+LeaderboardData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LocalLeaderboardsModel+LeaderboardType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalLeaderboardsModel_LeaderboardType {
    AllTime = 0i32,
    Daily = 1i32,
}
#[cfg(feature = "LocalLeaderboardsModel+LeaderboardType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType => ""
    ."LocalLeaderboardsModel/LeaderboardType"
);
#[cfg(feature = "LocalLeaderboardsModel+SavedLeaderboardsData")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalLeaderboardsModel_SavedLeaderboardsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _leaderboardsData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardData,
        >,
    >,
}
#[cfg(feature = "LocalLeaderboardsModel+SavedLeaderboardsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LocalLeaderboardsModel_SavedLeaderboardsData => ""
    ."LocalLeaderboardsModel/SavedLeaderboardsData"
);
#[cfg(feature = "LocalLeaderboardsModel+SavedLeaderboardsData")]
impl std::ops::Deref
for crate::GlobalNamespace::LocalLeaderboardsModel_SavedLeaderboardsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardsModel+SavedLeaderboardsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LocalLeaderboardsModel_SavedLeaderboardsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardsModel+SavedLeaderboardsData")]
impl crate::GlobalNamespace::LocalLeaderboardsModel_SavedLeaderboardsData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "LocalLeaderboardsModel+SavedLeaderboardsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalLeaderboardsModel_SavedLeaderboardsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LocalLeaderboardsModel+ScoreData")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalLeaderboardsModel_ScoreData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _score: i32,
    pub _playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _fullCombo: bool,
    pub _timestamp: i64,
}
#[cfg(feature = "LocalLeaderboardsModel+ScoreData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LocalLeaderboardsModel_ScoreData => ""
    ."LocalLeaderboardsModel/ScoreData"
);
#[cfg(feature = "LocalLeaderboardsModel+ScoreData")]
impl std::ops::Deref for crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardsModel+ScoreData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardsModel+ScoreData")]
impl crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "LocalLeaderboardsModel+ScoreData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
