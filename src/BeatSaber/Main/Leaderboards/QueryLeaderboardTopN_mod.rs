#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+QueryLeaderboardTopN")]
#[repr(C)]
#[derive(Debug)]
pub struct QueryLeaderboardTopN {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::QueryRequest,
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+QueryLeaderboardTopN")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Main::Leaderboards::QueryLeaderboardTopN
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.Leaderboards";
    const CLASS_NAME: &'static str = "QueryLeaderboardTopN";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "BeatSaber+Main+Leaderboards+QueryLeaderboardTopN")]
impl std::ops::Deref for crate::BeatSaber::Main::Leaderboards::QueryLeaderboardTopN {
    type Target = crate::OculusStudios::GraphQL::Client::QueryRequest;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+QueryLeaderboardTopN")]
impl std::ops::DerefMut for crate::BeatSaber::Main::Leaderboards::QueryLeaderboardTopN {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+QueryLeaderboardTopN")]
impl crate::BeatSaber::Main::Leaderboards::QueryLeaderboardTopN {
    pub const kDefaultAmountEntries: i64 = 10i64;
    pub fn CreateQuery(
        key: crate::GlobalNamespace::BeatmapKey,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        amountEntries: i64,
        order: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryOrder,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Main::GraphQL::Queries::BSLeaderboardTopN>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::BeatmapKey,
                            crate::GlobalNamespace::GameplayModifierMask,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryOrder,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Queries::BSLeaderboardTopN,
                        >,
                        4usize,
                    >("CreateQuery")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateQuery", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Queries::BSLeaderboardTopN,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (key, modifiers, amountEntries, order))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        key: crate::GlobalNamespace::BeatmapKey,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        amountEntries: i64,
        order: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryOrder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, modifiers, amountEntries, order))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        key: crate::GlobalNamespace::BeatmapKey,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        amountEntries: i64,
        order: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryOrder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::BeatmapKey,
                            crate::GlobalNamespace::GameplayModifierMask,
                            i64,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryOrder,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (key, modifiers, amountEntries, order))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+QueryLeaderboardTopN")]
impl quest_hook::libil2cpp::ObjectType
    for crate::BeatSaber::Main::Leaderboards::QueryLeaderboardTopN
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
