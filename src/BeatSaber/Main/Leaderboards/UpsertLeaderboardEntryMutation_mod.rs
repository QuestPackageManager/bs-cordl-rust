#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+UpsertLeaderboardEntryMutation")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct UpsertLeaderboardEntryMutation {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::MutationRequest_1<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryUpsertData,
        >,
    >,
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+UpsertLeaderboardEntryMutation")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Main::Leaderboards::UpsertLeaderboardEntryMutation
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.Leaderboards";
    const CLASS_NAME: &'static str = "UpsertLeaderboardEntryMutation";
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
#[cfg(feature = "BeatSaber+Main+Leaderboards+UpsertLeaderboardEntryMutation")]
impl std::ops::Deref for crate::BeatSaber::Main::Leaderboards::UpsertLeaderboardEntryMutation {
    type Target = crate::OculusStudios::GraphQL::Client::MutationRequest_1<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryUpsertData,
        >,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+UpsertLeaderboardEntryMutation")]
impl std::ops::DerefMut for crate::BeatSaber::Main::Leaderboards::UpsertLeaderboardEntryMutation {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+UpsertLeaderboardEntryMutation")]
impl crate::BeatSaber::Main::Leaderboards::UpsertLeaderboardEntryMutation {
    pub fn ConvertInputData(
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        modifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryUpsertData,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCompletionResults,
                            >,
                            crate::GlobalNamespace::BeatmapKey,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryUpsertData,
                        >,
                        3usize,
                    >("ConvertInputData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertInputData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesBeatmapLeaderboardEntryUpsertData,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (levelCompletionResults, beatmapKey, modifiers))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        results: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        modifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (results, beatmapKey, modifiers))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        results: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        modifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
                        crate::GlobalNamespace::BeatmapKey,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (results, beatmapKey, modifiers))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+UpsertLeaderboardEntryMutation")]
impl quest_hook::libil2cpp::ObjectType
    for crate::BeatSaber::Main::Leaderboards::UpsertLeaderboardEntryMutation
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
