#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion4::BeatmapDataLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion4";
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
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::BeatmapDataLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::BeatmapDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
impl crate::BeatmapDataLoaderVersion4::BeatmapDataLoader {
    pub fn ConvertBasicEvents(
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        environmentKeywords: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion4::LightshowSaveData,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BpmTimeProcessor>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::EnvironmentKeywords,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatmapLightEventConverter,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("ConvertBasicEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertBasicEvents", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapData,
                        lightshowSaveData,
                        bpmTimeProcessor,
                        environmentKeywords,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertBeatmapObjects(
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        beatmapSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::BeatmapSaveData,
        >,
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion4::BeatmapSaveData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion4::LightshowSaveData,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BpmTimeProcessor>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ConvertBeatmapObjects")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertBeatmapObjects",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (beatmapData, beatmapSaveData, lightshowSaveData, bpmTimeProcessor),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertEventBoxGroups(
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion4::LightshowSaveData,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BpmTimeProcessor>,
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertEventBoxGroups",
                    5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapData,
                        lightshowSaveData,
                        bpmTimeProcessor,
                        environmentLightGroups,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataBasicInfoFromSaveDataJson(
        beatmapDataJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
                1usize,
            >("GetBeatmapDataBasicInfoFromSaveDataJson")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetBeatmapDataBasicInfoFromSaveDataJson", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataBasicInfo,
        > = unsafe { method.invoke_unchecked((), (beatmapDataJson))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetBeatmapDataBasicInfoFromSaveDataJsonAsync", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
            >,
        > = unsafe { method.invoke_unchecked((), (beatmapJson))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataFromSaveData(
        audioSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapLevelSaveDataVersion4::AudioSaveData,
        >,
        beatmapSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::BeatmapSaveData,
        >,
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        defaultLightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        loadingForDesignatedEnvironment: bool,
        environmentKeywords: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        >,
        environmentLightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapLevelSaveDataVersion4::AudioSaveData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion4::BeatmapSaveData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion4::LightshowSaveData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion4::LightshowSaveData,
                    >,
                    crate::GlobalNamespace::BeatmapDifficulty,
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::EnvironmentKeywords,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IEnvironmentLightGroups,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlayerSpecificSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatmapLightEventConverter,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                11usize,
            >("GetBeatmapDataFromSaveData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetBeatmapDataFromSaveData", 11usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        audioSaveData,
                        beatmapSaveData,
                        lightshowSaveData,
                        defaultLightshowSaveData,
                        beatmapDifficulty,
                        loadingForDesignatedEnvironment,
                        environmentKeywords,
                        environmentLightGroups,
                        gameplayModifiers,
                        playerSpecificSettings,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataFromSaveDataJson(
        audioDataJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lightshowJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultLightshowJson: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        loadingForDesignatedEnvironment: bool,
        targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::GlobalNamespace::BeatmapDifficulty,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEnvironmentInfo>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEnvironmentInfo>,
                    crate::GlobalNamespace::BeatmapLevelDataVersion,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlayerSpecificSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatmapLightEventConverter,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                12usize,
            >("GetBeatmapDataFromSaveDataJson")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetBeatmapDataFromSaveDataJson", 12usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        audioDataJson,
                        beatmapJson,
                        lightshowJson,
                        defaultLightshowJson,
                        beatmapDifficulty,
                        loadingForDesignatedEnvironment,
                        targetEnvironmentInfo,
                        originalEnvironmentInfo,
                        beatmapLevelDataVersion,
                        gameplayModifiers,
                        playerSpecificSettings,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataFromSaveDataJsonAsync(
        audioDataJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lightshowJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultLightshowJson: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        loadingForDesignatedEnvironment: bool,
        targetEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        originalEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::GlobalNamespace::BeatmapDifficulty,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEnvironmentInfo>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEnvironmentInfo>,
                    crate::GlobalNamespace::BeatmapLevelDataVersion,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlayerSpecificSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatmapLightEventConverter,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                    >,
                >,
                12usize,
            >("GetBeatmapDataFromSaveDataJsonAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetBeatmapDataFromSaveDataJsonAsync", 12usize
                )
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
                        audioDataJson,
                        beatmapJson,
                        lightshowJson,
                        defaultLightshowJson,
                        beatmapDifficulty,
                        loadingForDesignatedEnvironment,
                        targetEnvironmentInfo,
                        originalEnvironmentInfo,
                        beatmapLevelDataVersion,
                        gameplayModifiers,
                        playerSpecificSettings,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadLightshow(
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        environmentKeywords: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        >,
        environmentLightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatmapDataLoaderVersion4::BeatmapDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatmapSaveDataVersion4::LightshowSaveData,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BpmTimeProcessor>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::EnvironmentKeywords,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IEnvironmentLightGroups,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBeatmapLightEventConverter,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("LoadLightshow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader as
                    quest_hook::libil2cpp::Type > ::class(), "LoadLightshow", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapData,
                        lightshowSaveData,
                        bpmTimeProcessor,
                        environmentKeywords,
                        environmentLightGroups,
                        lightEventConverter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::BeatmapDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
