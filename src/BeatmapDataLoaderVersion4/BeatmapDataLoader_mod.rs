#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader =>
    "BeatmapDataLoaderVersion4"."BeatmapDataLoader"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::BeatmapDataLoader {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertBasicEvents",
                (
                    beatmapData,
                    lightshowSaveData,
                    bpmTimeProcessor,
                    environmentKeywords,
                    lightEventConverter,
                ),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertBeatmapObjects",
                (beatmapData, beatmapSaveData, lightshowSaveData, bpmTimeProcessor),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertEventBoxGroups",
                (
                    beatmapData,
                    lightshowSaveData,
                    bpmTimeProcessor,
                    environmentLightGroups,
                    lightEventConverter,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataBasicInfoFromSaveDataJson(
        beatmapDataJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataBasicInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBeatmapDataBasicInfoFromSaveDataJson", (beatmapDataJson))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataBasicInfoFromSaveDataJsonAsync(
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBeatmapDataBasicInfoFromSaveDataJsonAsync", (beatmapJson))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetBeatmapDataFromSaveData",
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
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetBeatmapDataFromSaveDataJson",
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
            )?;
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
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetBeatmapDataFromSaveDataJsonAsync",
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
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadLightshow",
                (
                    beatmapData,
                    lightshowSaveData,
                    bpmTimeProcessor,
                    environmentKeywords,
                    environmentLightGroups,
                    lightEventConverter,
                ),
            )?;
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
