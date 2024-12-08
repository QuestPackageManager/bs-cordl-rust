#[cfg(feature = "BeatmapDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader {
    __cordl_parent: crate::System::Object,
    pub _lastUsedBeatmapDataCache: LastUsedBeatmapDataCache,
}
#[cfg(feature = "BeatmapDataLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapDataLoader => ""."BeatmapDataLoader"
);
#[cfg(feature = "BeatmapDataLoader")]
impl std::ops::Deref for BeatmapDataLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoader")]
impl std::ops::DerefMut for BeatmapDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoader")]
impl BeatmapDataLoader {
    #[cfg(feature = "BeatmapDataLoader+_LoadBasicBeatmapDataAsync_d__1")]
    pub type _LoadBasicBeatmapDataAsync_d__1 = crate::GlobalNamespace::BeatmapDataLoader__LoadBasicBeatmapDataAsync_d__1;
    #[cfg(feature = "BeatmapDataLoader+_LoadBeatmapDataAsync_d__4")]
    pub type _LoadBeatmapDataAsync_d__4 = crate::GlobalNamespace::BeatmapDataLoader__LoadBeatmapDataAsync_d__4;
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
    pub fn LoadBeatmapDataAsync(
        &mut self,
        beatmapLevelData: *mut IBeatmapLevelData,
        beatmapKey: BeatmapKey,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentInfo: *mut IEnvironmentInfo,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
        enableBeatmapDataCaching: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut IReadonlyBeatmapData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut IReadonlyBeatmapData,
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
    pub fn LoadBasicBeatmapDataAsync(
        &mut self,
        beatmapLevelData: *mut IBeatmapLevelData,
        beatmapKey: BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut BeatmapDataBasicInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut BeatmapDataBasicInfo,
        > = __cordl_object
            .invoke("LoadBasicBeatmapDataAsync", (beatmapLevelData, beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn LoadBasicBeatmapData_IBeatmapLevelData_ByRefMut0(
        &mut self,
        beatmapLevelData: *mut IBeatmapLevelData,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapDataBasicInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataBasicInfo = __cordl_object
            .invoke("LoadBasicBeatmapData", (beatmapLevelData, beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn LoadBasicBeatmapData_String1(
        &mut self,
        beatmapJson: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapDataBasicInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataBasicInfo = __cordl_object
            .invoke("LoadBasicBeatmapData", (beatmapJson))?;
        Ok(__cordl_ret)
    }
    pub fn LoadBeatmapData(
        &mut self,
        beatmapLevelData: *mut IBeatmapLevelData,
        beatmapKey: BeatmapKey,
        startBpm: f32,
        loadingForDesignatedEnvironment: bool,
        environmentInfo: *mut IEnvironmentInfo,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
    ) -> quest_hook::libil2cpp::Result<*mut IReadonlyBeatmapData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IReadonlyBeatmapData = __cordl_object
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoader")]
impl quest_hook::libil2cpp::ObjectType for BeatmapDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
