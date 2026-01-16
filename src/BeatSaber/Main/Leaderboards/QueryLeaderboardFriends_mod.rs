#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+QueryLeaderboardFriends")]
#[repr(C)]
#[derive(Debug)]
pub struct QueryLeaderboardFriends {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::QueryRequest,
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+QueryLeaderboardFriends")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::Leaderboards::QueryLeaderboardFriends {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.Leaderboards";
    const CLASS_NAME: &'static str = "QueryLeaderboardFriends";
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
#[cfg(feature = "BeatSaber+Main+Leaderboards+QueryLeaderboardFriends")]
impl std::ops::Deref for crate::BeatSaber::Main::Leaderboards::QueryLeaderboardFriends {
    type Target = crate::OculusStudios::GraphQL::Client::QueryRequest;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+QueryLeaderboardFriends")]
impl std::ops::DerefMut
for crate::BeatSaber::Main::Leaderboards::QueryLeaderboardFriends {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+QueryLeaderboardFriends")]
impl crate::BeatSaber::Main::Leaderboards::QueryLeaderboardFriends {
    pub const kDefaultAmountEntries: i64 = 10i64;
    pub fn CreateQuery(
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        key: crate::GlobalNamespace::BeatmapKey,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        amountEntries: i64,
        order: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryOrder,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Queries::BSLeaderboardFriends,
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
                            crate::GlobalNamespace::BeatmapKey,
                            crate::GlobalNamespace::GameplayModifierMask,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryOrder,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Queries::BSLeaderboardFriends,
                        >,
                        5usize,
                    >("CreateQuery")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateQuery", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Queries::BSLeaderboardFriends,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (userId, key, modifiers, amountEntries, order))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        key: crate::GlobalNamespace::BeatmapKey,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        amountEntries: i64,
        order: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryOrder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (userId, key, modifiers, amountEntries, order))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        key: crate::GlobalNamespace::BeatmapKey,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        amountEntries: i64,
        order: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryOrder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::BeatmapKey,
                            crate::GlobalNamespace::GameplayModifierMask,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryOrder,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (userId, key, modifiers, amountEntries, order))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+QueryLeaderboardFriends")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::Leaderboards::QueryLeaderboardFriends {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
