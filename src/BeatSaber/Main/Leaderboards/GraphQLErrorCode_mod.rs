#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+GraphQLErrorCode")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct GraphQLErrorCode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+GraphQLErrorCode")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Main::Leaderboards::GraphQLErrorCode {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.Leaderboards";
    const CLASS_NAME: &'static str = "GraphQLErrorCode";
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
#[cfg(feature = "BeatSaber+Main+Leaderboards+GraphQLErrorCode")]
impl std::ops::Deref for crate::BeatSaber::Main::Leaderboards::GraphQLErrorCode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+GraphQLErrorCode")]
impl std::ops::DerefMut for crate::BeatSaber::Main::Leaderboards::GraphQLErrorCode {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+GraphQLErrorCode")]
impl crate::BeatSaber::Main::Leaderboards::GraphQLErrorCode {
    pub const BEAT_GAMES__CLIENT_VERSION_DEPRECATED: i32 = 3789024i32;
    pub const BEAT_GAMES__GLOBAL_KILL_SWITCH_ON: i32 = 3789003i32;
    pub const BEAT_GAMES__ONBOARDING_EXPERIENCE_FROM_WRONG_DATA_ENVIRONMENT: i32 = 3789034i32;
    pub const BEAT_GAMES__SCHEDULE_CONFLICTING_CONTENT_VERSIONS_WITH_SAME_MINIMUM_CLIENT_VERSION:
        i32 = 3789036i32;
    pub const BEAT_GAMES__SUBMIT_BEATMAP_ATTEMPT_KILL_SWITCH_ON: i32 = 3789007i32;
    pub const BEAT_GAMES__UNSUPPORTED_FIELD_TYPE_FOR_STRING_SERIALIZATION: i32 = 3789021i32;
    pub const BEAT_GAMES__UPSERT_LEADERBOARD_ENTRY_KILLSWITCH_ON: i32 = 3789025i32;
    pub const BEAT_GAMES__USER_IS_DELETED: i32 = 3789026i32;
    pub const MAX_ERROR_CODE: i32 = 3789040i32;
    pub const MIN_ERROR_CODE: i32 = 3789003i32;
    pub fn IsBeatGamesErrorCode_Il2CppString0(
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        bool,
                        1usize,
                    >("IsBeatGamesErrorCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsBeatGamesErrorCode", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (code))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsBeatGamesErrorCode_i32_1(code: i32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), bool, 1usize>("IsBeatGamesErrorCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsBeatGamesErrorCode",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (code))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+GraphQLErrorCode")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Main::Leaderboards::GraphQLErrorCode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
