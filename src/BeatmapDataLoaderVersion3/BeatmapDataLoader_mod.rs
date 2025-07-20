#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader {
    pub const kDefaultNumberOfLines: i32 = 4i32;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
    pub type BasicEventConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter"
    )]
    pub type BeatmapDataItemConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
    pub type BombNoteConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
    pub type BpmEventConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
    pub type BurstSliderConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter"
    )]
    pub type ColorBoostEventConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
    pub type ColorNoteConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor"
    )]
    pub type FloatVfxBaseDataConvertor = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter"
    )]
    pub type FloatVfxEventBoxConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
    pub type IndexFilterConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor"
    )]
    pub type IntVfxBaseDataConvertor = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter"
    )]
    pub type IntVfxEventBoxConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
    )]
    pub type LightColoBaseDataConvertor = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
    )]
    pub type LightColorEventBoxConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
    )]
    pub type LightRotationBaseDataConvertor = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
    )]
    pub type LightRotationEventBoxConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
    )]
    pub type LightTranslationBaseDataConvertor = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
    )]
    pub type LightTranslationEventBoxConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
    pub type ObstacleConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
    pub type SliderConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
    pub type SpecialEventsFilter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
    pub type WaypointConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter;
    pub fn ConvertBasicEvents(
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        beatmapSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::BeatmapSaveData,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
        environmentKeywords: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::BeatmapSaveData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::EnvironmentKeywords,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ConvertBasicEvents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertBasicEvents", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapData,
                        beatmapSaveData,
                        bpmTimeProcessor,
                        rotationTimeProcessor,
                        environmentKeywords,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertEventBoxGroups(
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        beatmapSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::BeatmapSaveData,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        environmentLightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::BeatmapSaveData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IEnvironmentLightGroups,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapLightEventConverter,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ConvertEventBoxGroups")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertEventBoxGroups", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapData,
                        beatmapSaveData,
                        bpmTimeProcessor,
                        environmentLightGroups,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataBasicInfoFromSaveDataJson(
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataBasicInfo,
                        >,
                        1usize,
                    >("GetBeatmapDataBasicInfoFromSaveDataJson")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetBeatmapDataBasicInfoFromSaveDataJson",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataBasicInfo,
        > = unsafe { method.invoke_unchecked((), (beatmapJson))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataBasicInfoFromSaveDataJsonAsync(
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapDataBasicInfo,
                                >,
                            >,
                        >,
                        1usize,
                    >("GetBeatmapDataBasicInfoFromSaveDataJsonAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "GetBeatmapDataBasicInfoFromSaveDataJsonAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
            >,
        > = unsafe { method.invoke_unchecked((), (beatmapJson))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataFromSaveData(
        beatmapSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::BeatmapSaveData,
        >,
        defaultLightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentKeywords: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        >,
        environmentLightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
        stopwatch: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::Stopwatch>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::BeatmapSaveData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion4::LightshowSaveData,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                            f32,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::EnvironmentKeywords,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IEnvironmentLightGroups,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSpecificSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapLightEventConverter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Diagnostics::Stopwatch,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                        10usize,
                    >("GetBeatmapDataFromSaveData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetBeatmapDataFromSaveData", 10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapSaveData,
                        defaultLightshowSaveData,
                        beatmapDifficulty,
                        startBpm,
                        loadingForDesignatedEnvironment,
                        environmentKeywords,
                        environmentLightGroups,
                        playerSpecificSettings,
                        lightEventConverter,
                        stopwatch,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataFromSaveDataJson(
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultLightshowJson: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                            f32,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IEnvironmentInfo,
                            >,
                            crate::GlobalNamespace::BeatmapLevelDataVersion,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSpecificSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapLightEventConverter,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                        9usize,
                    >("GetBeatmapDataFromSaveDataJson")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetBeatmapDataFromSaveDataJson", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapJson,
                        defaultLightshowJson,
                        beatmapDifficulty,
                        startBpm,
                        loadingForDesignatedEnvironment,
                        environmentInfo,
                        beatmapLevelDataVersion,
                        playerSpecificSettings,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataFromSaveDataJsonAsync(
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultLightshowJson: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                            f32,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IEnvironmentInfo,
                            >,
                            crate::GlobalNamespace::BeatmapLevelDataVersion,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerSpecificSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapLightEventConverter,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapData,
                                >,
                            >,
                        >,
                        9usize,
                    >("GetBeatmapDataFromSaveDataJsonAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetBeatmapDataFromSaveDataJsonAsync", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
            >,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapJson,
                        defaultLightshowJson,
                        beatmapDifficulty,
                        startBpm,
                        loadingForDesignatedEnvironment,
                        environmentInfo,
                        beatmapLevelDataVersion,
                        playerSpecificSettings,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BasicEventConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
    pub _specialEventsFilter: quest_hook::libil2cpp::Gc<
        crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/BasicEventConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter {
    pub fn Convert(
        &mut self,
        basicEventSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::BasicEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::BasicEventData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        > = unsafe { method.invoke_unchecked(self, (basicEventSaveData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
        specialEventsFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (bpmTimeProcessor, rotationTimeProcessor, specialEventsFilter),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
        specialEventsFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (bpmTimeProcessor, rotationTimeProcessor, specialEventsFilter),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BeatmapDataItemConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bpmTimeProcessor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BpmTimeProcessor,
    >,
    pub _rotationTimeProcessor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RotationTimeProcessor,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/BeatmapDataItemConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter {
    pub fn BeatToRotation(&mut self, beat: f32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(f32), i32, 1usize>("BeatToRotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "BeatToRotation", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (beat))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeatToTime(&mut self, beat: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(f32), f32, 1usize>("BeatToTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "BeatToTime", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (beat))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BombNoteConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/BombNoteConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter {
    pub fn Convert(
        &mut self,
        bombNoteSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::BombNoteData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::BombNoteData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapObjectData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        > = unsafe { method.invoke_unchecked(self, (bombNoteSaveData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BpmEventConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/BpmEventConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter {
    pub fn Convert(
        &mut self,
        bpmChangeEventSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::BpmChangeEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BPMChangeBeatmapEventData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::BpmChangeEventData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BPMChangeBeatmapEventData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BPMChangeBeatmapEventData,
        > = unsafe { method.invoke_unchecked(self, (bpmChangeEventSaveData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BurstSliderConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/BurstSliderConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter {
    pub fn Convert(
        &mut self,
        sliderSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::BurstSliderData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::BurstSliderData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapObjectData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        > = unsafe { method.invoke_unchecked(self, (sliderSaveData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_ColorBoostEventConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/ColorBoostEventConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter {
    pub fn Convert(
        &mut self,
        colorBoostEventSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::ColorBoostEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::ColorBoostEventData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        > = unsafe { method.invoke_unchecked(self, (colorBoostEventSaveData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_ColorNoteConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/ColorNoteConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter {
    pub fn Convert(
        &mut self,
        colorNoteSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::ColorNoteData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::ColorNoteData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapObjectData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        > = unsafe { method.invoke_unchecked(self, (colorNoteSaveData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_FloatVfxBaseDataConvertor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/FloatVfxBaseDataConvertor";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor {
    pub fn Convert(
        vfxEventBaseData: i32,
        collection: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::FxEventsCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FloatFxBaseData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::FxEventsCollection,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::FloatFxBaseData,
                        >,
                        2usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxBaseData,
        > = unsafe { method.invoke_unchecked((), (vfxEventBaseData, collection))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_FloatVfxEventBoxConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _fxEventsCollection: quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataVersion3::FxEventsCollection,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/FloatVfxEventBoxConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter {
    pub fn Convert(
        &mut self,
        saveData: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::FxEventBox>,
        lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILightGroup>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::FxEventBox,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ILightGroup,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventDataBox,
                        >,
                        2usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBox,
        > = unsafe { method.invoke_unchecked(self, (saveData, lightGroup))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        fxEventsCollection: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::FxEventsCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fxEventsCollection))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        fxEventsCollection: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::FxEventsCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::FxEventsCollection,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (fxEventsCollection))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_IndexFilterConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/IndexFilterConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter {
    pub fn Convert(
        indexFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::IndexFilter,
        >,
        groupSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::IndexFilter,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
                        2usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IndexFilter,
        > = unsafe { method.invoke_unchecked((), (indexFilter, groupSize))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_IntVfxBaseDataConvertor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/IntVfxBaseDataConvertor";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor {
    pub fn Convert(
        vfxEventBaseData: i32,
        collection: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::FxEventsCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IntFxBaseData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::FxEventsCollection,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IntFxBaseData>,
                        2usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IntFxBaseData,
        > = unsafe { method.invoke_unchecked((), (vfxEventBaseData, collection))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_IntVfxEventBoxConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _fxEventsCollection: quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataVersion3::FxEventsCollection,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/IntVfxEventBoxConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter {
    pub fn Convert(
        &mut self,
        saveData: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::FxEventBox>,
        lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILightGroup>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::FxEventBox,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ILightGroup,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventDataBox,
                        >,
                        2usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBox,
        > = unsafe { method.invoke_unchecked(self, (saveData, lightGroup))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        fxEventsCollection: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::FxEventsCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fxEventsCollection))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        fxEventsCollection: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::FxEventsCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::FxEventsCollection,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (fxEventsCollection))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightColoBaseDataConvertor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/LightColoBaseDataConvertor";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor {
    pub fn Convert(
        saveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightColorBaseData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightColorBaseData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::LightColorBaseData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LightColorBaseData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightColorBaseData,
        > = unsafe { method.invoke_unchecked((), (saveData))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightColorEventBoxConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/LightColorEventBoxConverter";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter {
    pub fn Convert(
        saveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightColorEventBox,
        >,
        lightGroupData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILightGroup>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::LightColorEventBox,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ILightGroup,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventDataBox,
                        >,
                        2usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBox,
        > = unsafe { method.invoke_unchecked((), (saveData, lightGroupData))? };
        Ok(__cordl_ret.into())
    }
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightRotationBaseDataConvertor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/LightRotationBaseDataConvertor";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor {
    pub fn Convert(
        saveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightRotationBaseData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightRotationBaseData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::LightRotationBaseData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LightRotationBaseData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightRotationBaseData,
        > = unsafe { method.invoke_unchecked((), (saveData))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightRotationEventBoxConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/LightRotationEventBoxConverter";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter {
    pub fn Convert(
        saveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightRotationEventBox,
        >,
        lightGroupData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILightGroup>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::LightRotationEventBox,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ILightGroup,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventDataBox,
                        >,
                        2usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBox,
        > = unsafe { method.invoke_unchecked((), (saveData, lightGroupData))? };
        Ok(__cordl_ret.into())
    }
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightTranslationBaseDataConvertor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/LightTranslationBaseDataConvertor";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor {
    pub fn Convert(
        saveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightTranslationBaseData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightTranslationBaseData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::LightTranslationBaseData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LightTranslationBaseData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightTranslationBaseData,
        > = unsafe { method.invoke_unchecked((), (saveData))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightTranslationEventBoxConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/LightTranslationEventBoxConverter";
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
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter {
    pub fn Convert(
        saveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightTranslationEventBox,
        >,
        lightGroupData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILightGroup>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion3::LightTranslationEventBox,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ILightGroup,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventDataBox,
                        >,
                        2usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBox,
        > = unsafe { method.invoke_unchecked((), (saveData, lightGroupData))? };
        Ok(__cordl_ret.into())
    }
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_ObstacleConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/ObstacleConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter {
    pub fn Convert(
        &mut self,
        obstacleSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::ObstacleData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::ObstacleData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapObjectData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        > = unsafe { method.invoke_unchecked(self, (obstacleSaveData))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetNoteLineLayer(
        lineLayer: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32),
                        crate::GlobalNamespace::NoteLineLayer,
                        1usize,
                    >("GetNoteLineLayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetNoteLineLayer", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = unsafe {
            method.invoke_unchecked((), (lineLayer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_SliderConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/SliderConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter {
    pub fn Convert(
        &mut self,
        sliderSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::SliderData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::SliderData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapObjectData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        > = unsafe { method.invoke_unchecked(self, (sliderSaveData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_SpecialEventsFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _eventTypesToFilter: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            crate::BeatmapSaveDataCommon::BeatmapEventType,
        >,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/SpecialEventsFilter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter {
    pub fn IsEventValid(
        &mut self,
        basicBeatmapEventType: crate::BeatmapSaveDataCommon::BeatmapEventType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::BeatmapSaveDataCommon::BeatmapEventType),
                        bool,
                        1usize,
                    >("IsEventValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsEventValid", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (basicBeatmapEventType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        basicEventTypesWithKeywords: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
        >,
        environmentKeywords: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (basicEventTypesWithKeywords, environmentKeywords))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        basicEventTypesWithKeywords: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
        >,
        environmentKeywords: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::EnvironmentKeywords,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (basicEventTypesWithKeywords, environmentKeywords),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_WaypointConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion3";
    const CLASS_NAME: &'static str = "BeatmapDataLoader/WaypointConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter {
    pub fn Convert(
        &mut self,
        waypointSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::WaypointData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::WaypointData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapObjectData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Convert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        > = unsafe { method.invoke_unchecked(self, (waypointSaveData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BpmTimeProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RotationTimeProcessor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bpmTimeProcessor, rotationTimeProcessor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
