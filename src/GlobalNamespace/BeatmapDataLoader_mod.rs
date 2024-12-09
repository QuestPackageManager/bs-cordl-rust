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
    #[cfg(feature = "BeatmapDataLoader+_LoadBasicBeatmapDataAsync_d__1")]
    pub type _LoadBasicBeatmapDataAsync_d__1 = crate::GlobalNamespace::BeatmapDataLoader__LoadBasicBeatmapDataAsync_d__1;
    #[cfg(feature = "BeatmapDataLoader+_LoadBeatmapDataAsync_d__4")]
    pub type _LoadBeatmapDataAsync_d__4 = crate::GlobalNamespace::BeatmapDataLoader__LoadBeatmapDataAsync_d__4;
    pub fn LoadBasicBeatmapDataAsync(
        &mut self,
        beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::BeatmapDataBasicInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::BeatmapDataBasicInfo,
        > = __cordl_object
            .invoke("LoadBasicBeatmapDataAsync", (beatmapLevelData, beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn LoadBasicBeatmapData_IBeatmapLevelData_ByRefMut0(
        &mut self,
        beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapDataBasicInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapDataBasicInfo = __cordl_object
            .invoke("LoadBasicBeatmapData", (beatmapLevelData, beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn LoadBasicBeatmapData_Il2CppString1(
        &mut self,
        beatmapJson: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapDataBasicInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapDataBasicInfo = __cordl_object
            .invoke("LoadBasicBeatmapData", (beatmapJson))?;
        Ok(__cordl_ret)
    }
    pub fn LoadBeatmapData(
        &mut self,
        beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentInfo: *mut crate::GlobalNamespace::IEnvironmentInfo,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::IReadonlyBeatmapData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IReadonlyBeatmapData = __cordl_object
            .invoke(
                "LoadBeatmapData",
                (
                    beatmapLevelData,
                    beatmapKey,
                    startBpm,
                    loadingForDesignatedEnvironment,
                    environmentInfo,
                    beatmapLevelDataVersion,
                    gameplayModifiers,
                    playerSpecificSettings,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadBeatmapDataAsync(
        &mut self,
        beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentInfo: *mut crate::GlobalNamespace::IEnvironmentInfo,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        enableBeatmapDataCaching: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::IReadonlyBeatmapData,
        > = __cordl_object
            .invoke(
                "LoadBeatmapDataAsync",
                (
                    beatmapLevelData,
                    beatmapKey,
                    startBpm,
                    loadingForDesignatedEnvironment,
                    environmentInfo,
                    beatmapLevelDataVersion,
                    gameplayModifiers,
                    playerSpecificSettings,
                    enableBeatmapDataCaching,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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
