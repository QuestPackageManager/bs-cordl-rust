#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+LeaderboardHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+LeaderboardHelpers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::Leaderboards::LeaderboardHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.Leaderboards";
    const CLASS_NAME: &'static str = "LeaderboardHelpers";
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
#[cfg(feature = "BeatSaber+Main+Leaderboards+LeaderboardHelpers")]
impl std::ops::Deref for crate::BeatSaber::Main::Leaderboards::LeaderboardHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+LeaderboardHelpers")]
impl std::ops::DerefMut for crate::BeatSaber::Main::Leaderboards::LeaderboardHelpers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+LeaderboardHelpers")]
impl crate::BeatSaber::Main::Leaderboards::LeaderboardHelpers {
    pub fn ConvertCharacteristic(
        characteristicName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapCharacteristic,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapCharacteristic,
                        1usize,
                    >("ConvertCharacteristic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertCharacteristic", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapCharacteristic = unsafe {
            cordl_method_info.invoke_unchecked((), (characteristicName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertDifficulty(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapDifficulty,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::BeatmapDifficulty),
                        crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapDifficulty,
                        1usize,
                    >("ConvertDifficulty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertDifficulty", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapDifficulty = unsafe {
            cordl_method_info.invoke_unchecked((), (difficulty))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+LeaderboardHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::Leaderboards::LeaderboardHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
