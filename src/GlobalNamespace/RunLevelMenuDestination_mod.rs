#[cfg(feature = "RunLevelMenuDestination")]
#[repr(C)]
#[derive(Debug)]
pub struct RunLevelMenuDestination {
    __cordl_parent: crate::GlobalNamespace::MenuDestination,
    pub beatmapLevelPack: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelPack,
    >,
    pub beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    pub beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub beatmapCharacteristic: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
    pub gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiers,
    >,
    pub practice: bool,
    pub startSongTime: f32,
    pub songSpeedMultiplier: f32,
    pub overrideEnvironments: bool,
    pub environmentType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub quitAppAfterRun: bool,
}
#[cfg(feature = "RunLevelMenuDestination")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RunLevelMenuDestination {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RunLevelMenuDestination";
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
#[cfg(feature = "RunLevelMenuDestination")]
impl std::ops::Deref for crate::GlobalNamespace::RunLevelMenuDestination {
    type Target = crate::GlobalNamespace::MenuDestination;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RunLevelMenuDestination")]
impl std::ops::DerefMut for crate::GlobalNamespace::RunLevelMenuDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RunLevelMenuDestination")]
impl crate::GlobalNamespace::RunLevelMenuDestination {
    pub fn New(
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        practice: bool,
        startSongTime: f32,
        songSpeedMultiplier: f32,
        overrideEnvironments: bool,
        environmentType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quitAppAfterRun: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapLevelPack,
                    beatmapLevel,
                    beatmapDifficulty,
                    beatmapCharacteristic,
                    gameplayModifiers,
                    practice,
                    startSongTime,
                    songSpeedMultiplier,
                    overrideEnvironments,
                    environmentType,
                    environmentName,
                    quitAppAfterRun,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        practice: bool,
        startSongTime: f32,
        songSpeedMultiplier: f32,
        overrideEnvironments: bool,
        environmentType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quitAppAfterRun: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelPack,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCharacteristicSO,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                            bool,
                            f32,
                            f32,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        beatmapLevelPack,
                        beatmapLevel,
                        beatmapDifficulty,
                        beatmapCharacteristic,
                        gameplayModifiers,
                        practice,
                        startSongTime,
                        songSpeedMultiplier,
                        overrideEnvironments,
                        environmentType,
                        environmentName,
                        quitAppAfterRun,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RunLevelMenuDestination")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RunLevelMenuDestination {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
