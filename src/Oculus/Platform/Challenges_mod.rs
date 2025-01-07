#[cfg(feature = "Oculus+Platform+Challenges")]
#[repr(C)]
#[derive(Debug)]
pub struct Challenges {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Challenges")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Challenges {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Challenges";
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
#[cfg(feature = "Oculus+Platform+Challenges")]
impl std::ops::Deref for crate::Oculus::Platform::Challenges {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Challenges")]
impl std::ops::DerefMut for crate::Oculus::Platform::Challenges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Challenges")]
impl crate::Oculus::Platform::Challenges {
    pub fn Create(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        challengeOptions: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::ChallengeOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (leaderboardName, challengeOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeclineInvite(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeclineInvite", (challengeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn Delete(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Delete", (challengeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", (challengeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEntries(
        challengeID: u64,
        limit: i32,
        filter: crate::Oculus::Platform::LeaderboardFilterType,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEntries", (challengeID, limit, filter, startAt))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEntriesAfterRank(
        challengeID: u64,
        limit: i32,
        afterRank: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEntriesAfterRank", (challengeID, limit, afterRank))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEntriesByIds(
        challengeID: u64,
        limit: i32,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEntriesByIds", (challengeID, limit, startAt, userIDs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetList(
        challengeOptions: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::ChallengeOptions,
        >,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetList", (challengeOptions, limit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextChallenges(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextChallenges", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextEntries(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ChallengeEntryList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextEntries", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousChallenges(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreviousChallenges", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousEntries(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ChallengeEntryList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreviousEntries", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn Join(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Join", (challengeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn Leave(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Leave", (challengeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInfo(
        challengeID: u64,
        challengeOptions: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::ChallengeOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateInfo", (challengeID, challengeOptions))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Challenges")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Challenges {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
