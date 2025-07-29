#[cfg(feature = "cordl_class_Oculus+Platform+Leaderboards")]
#[repr(C)]
#[derive(Debug)]
pub struct Leaderboards {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Oculus+Platform+Leaderboards")]
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Leaderboards")]
impl std::ops::DerefMut for crate::Oculus::Platform::Leaderboards {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Leaderboards")]
impl crate::Oculus::Platform::Leaderboards {
    pub fn Get(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LeaderboardList>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::LeaderboardList,
                            >,
                        >,
                        1usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Get",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LeaderboardList>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (leaderboardName))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEntries(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        filter: crate::Oculus::Platform::LeaderboardFilterType,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::LeaderboardEntryList,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            crate::Oculus::Platform::LeaderboardFilterType,
                            crate::Oculus::Platform::LeaderboardStartAt,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::LeaderboardEntryList,
                            >,
                        >,
                        4usize,
                    >("GetEntries")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEntries", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::LeaderboardEntryList,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (leaderboardName, limit, filter, startAt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEntriesAfterRank(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        afterRank: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::LeaderboardEntryList,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            u64,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::LeaderboardEntryList,
                            >,
                        >,
                        3usize,
                    >("GetEntriesAfterRank")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEntriesAfterRank", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::LeaderboardEntryList,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (leaderboardName, limit, afterRank))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEntriesByIds(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        limit: i32,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::LeaderboardEntryList,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            crate::Oculus::Platform::LeaderboardStartAt,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u64>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::LeaderboardEntryList,
                            >,
                        >,
                        4usize,
                    >("GetEntriesByIds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEntriesByIds", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::LeaderboardEntryList,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (leaderboardName, limit, startAt, userIDs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextEntries(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LeaderboardEntryList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::LeaderboardEntryList,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::LeaderboardEntryList,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::LeaderboardEntryList,
                            >,
                        >,
                        1usize,
                    >("GetNextEntries")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNextEntries", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::LeaderboardEntryList,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (list))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextLeaderboardListPage(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LeaderboardList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LeaderboardList>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::LeaderboardList,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::LeaderboardList,
                            >,
                        >,
                        1usize,
                    >("GetNextLeaderboardListPage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNextLeaderboardListPage", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LeaderboardList>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (list))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousEntries(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LeaderboardEntryList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::LeaderboardEntryList,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::LeaderboardEntryList,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::LeaderboardEntryList,
                            >,
                        >,
                        1usize,
                    >("GetPreviousEntries")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPreviousEntries", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::LeaderboardEntryList,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (list))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteEntry(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i64,
        extraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<bool>,
                        4usize,
                    >("WriteEntry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteEntry", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<bool> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (leaderboardName, score, extraData, forceUpdate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteEntryWithSupplementaryMetric(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i64,
        supplementaryMetric: i64,
        extraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i64,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<bool>,
                        5usize,
                    >("WriteEntryWithSupplementaryMetric")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteEntryWithSupplementaryMetric", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<bool> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (leaderboardName, score, supplementaryMetric, extraData, forceUpdate),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+Leaderboards")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Leaderboards {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
