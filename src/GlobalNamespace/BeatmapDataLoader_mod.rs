#[cfg(feature = "BeatmapDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lastUsedBeatmapDataCache: crate::GlobalNamespace::LastUsedBeatmapDataCache,
}
#[cfg(feature = "BeatmapDataLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataLoader => ""
    ."BeatmapDataLoader"
);
#[cfg(feature = "BeatmapDataLoader")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoader")]
impl crate::GlobalNamespace::BeatmapDataLoader {
    pub fn LoadBasicBeatmapDataAsync(
        &mut self,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::BeatmapDataBasicInfo,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::BeatmapDataBasicInfo,
            >,
        > = __cordl_object
            .invoke("LoadBasicBeatmapDataAsync", (beatmapLevelData, beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBasicBeatmapData_IBeatmapLevelData_ByRefMut0(
        &mut self,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataBasicInfo,
        > = __cordl_object
            .invoke("LoadBasicBeatmapData", (beatmapLevelData, beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBasicBeatmapData_Il2CppString1(
        &mut self,
        beatmapJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataBasicInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataBasicInfo,
        > = __cordl_object.invoke("LoadBasicBeatmapData", (beatmapJson))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapData(
        &mut self,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        lightshowEnvironmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = __cordl_object
            .invoke(
                "LoadBeatmapData",
                (
                    beatmapLevelData,
                    beatmapKey,
                    startBpm,
                    loadingForDesignatedEnvironment,
                    environmentInfo,
                    lightshowEnvironmentInfo,
                    beatmapLevelDataVersion,
                    gameplayModifiers,
                    playerSpecificSettings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapDataAsync(
        &mut self,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        startBpm: f32,
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
        enableBeatmapDataCaching: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::IReadonlyBeatmapData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::IReadonlyBeatmapData,
            >,
        > = __cordl_object
            .invoke(
                "LoadBeatmapDataAsync",
                (
                    beatmapLevelData,
                    beatmapKey,
                    startBpm,
                    loadingForDesignatedEnvironment,
                    targetEnvironmentInfo,
                    originalEnvironmentInfo,
                    beatmapLevelDataVersion,
                    gameplayModifiers,
                    playerSpecificSettings,
                    enableBeatmapDataCaching,
                ),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
