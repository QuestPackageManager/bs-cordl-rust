#[cfg(feature = "Oculus+Platform+Leaderboards")]
#[repr(C)]
#[derive(Debug)]
pub struct Leaderboards {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Leaderboards")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Leaderboards {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Leaderboards";
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
#[cfg(feature = "Oculus+Platform+Leaderboards")]
impl std::ops::Deref for crate::Oculus::Platform::Leaderboards {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Leaderboards")]
impl std::ops::DerefMut for crate::Oculus::Platform::Leaderboards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Leaderboards")]
impl crate::Oculus::Platform::Leaderboards {
    pub fn Get(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Get", (leaderboardName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEntries(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        filter: crate::Oculus::Platform::LeaderboardFilterType,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEntries", (leaderboardName, limit, filter, startAt))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEntriesAfterRank(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        afterRank: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEntriesAfterRank", (leaderboardName, limit, afterRank))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEntriesByIds(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEntriesByIds", (leaderboardName, limit, startAt, userIDs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextEntries(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LeaderboardEntryList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextEntries", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextLeaderboardListPage(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LeaderboardList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextLeaderboardListPage", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousEntries(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LeaderboardEntryList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreviousEntries", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEntry(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i64,
        extraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request_1<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteEntry", (leaderboardName, score, extraData, forceUpdate))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEntryWithSupplementaryMetric(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i64,
        supplementaryMetric: i64,
        extraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request_1<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteEntryWithSupplementaryMetric",
                (leaderboardName, score, supplementaryMetric, extraData, forceUpdate),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Leaderboards")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Leaderboards {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
