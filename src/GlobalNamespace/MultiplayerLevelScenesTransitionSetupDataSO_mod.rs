#[cfg(feature = "MultiplayerLevelScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLevelScenesTransitionSetupDataSO {
    __cordl_parent: LevelScenesTransitionSetupDataSO,
    pub _multiplayerLevelSceneInfo: *mut SceneInfo,
    pub _gameCoreSceneInfo: *mut SceneInfo,
    pub _multiplayerEnvironmentInfo: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut EnvironmentInfoSO,
    >,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut MultiplayerLevelScenesTransitionSetupDataSO,
        *mut MultiplayerResultsData,
    >,
    pub didDisconnectEvent: *mut crate::System::Action_2<
        *mut MultiplayerLevelScenesTransitionSetupDataSO,
        DisconnectedReason,
    >,
    pub _gameMode_k__BackingField: *mut crate::System::String,
    pub _beatmapKey_k__BackingField: BeatmapKey,
    pub _beatmapLevel_k__BackingField: *mut BeatmapLevel,
    pub _usingOverrideColorScheme_k__BackingField: bool,
    pub _colorScheme_k__BackingField: *mut ColorScheme,
    pub _beatmapLevelData_k__BackingField: *mut IBeatmapLevelData,
    pub _loadedMultiplayerEnvironmentInfo: *mut EnvironmentInfoSO,
}
#[cfg(feature = "MultiplayerLevelScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLevelScenesTransitionSetupDataSO => ""
    ."MultiplayerLevelScenesTransitionSetupDataSO"
);
#[cfg(feature = "MultiplayerLevelScenesTransitionSetupDataSO")]
impl std::ops::Deref for MultiplayerLevelScenesTransitionSetupDataSO {
    type Target = LevelScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for MultiplayerLevelScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelScenesTransitionSetupDataSO")]
impl MultiplayerLevelScenesTransitionSetupDataSO {
    pub fn set_beatmapKey(
        &mut self,
        value: BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapKey", (value))?;
        Ok(__cordl_ret)
    }
    pub fn InitColorInfo(
        &mut self,
        overrideColorScheme: *mut ColorScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitColorInfo", (overrideColorScheme))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerLevelScenesTransitionSetupDataSO,
            *mut MultiplayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_usingOverrideColorScheme(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_usingOverrideColorScheme", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapLevelData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IBeatmapLevelData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IBeatmapLevelData = __cordl_object
            .invoke("get_beatmapLevelData", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didDisconnectEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerLevelScenesTransitionSetupDataSO,
            DisconnectedReason,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didDisconnectEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapLevelData(
        &mut self,
        value: *mut IBeatmapLevelData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapLevelData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapLevel(
        &mut self,
        value: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapLevel", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_gameMode(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevel = __cordl_object
            .invoke("get_beatmapLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        gameMode: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        beatmapLevelData: *mut IBeatmapLevelData,
        overrideColorScheme: *mut ColorScheme,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
        practiceSettings: *mut PracticeSettings,
        audioClipAsyncLoader: *mut AudioClipAsyncLoader,
        performancePreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        beatmapDataLoader: *mut BeatmapDataLoader,
        useTestNoteCutSoundEffects: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    gameMode,
                    beatmapKey,
                    beatmapLevel,
                    beatmapLevelData,
                    overrideColorScheme,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    audioClipAsyncLoader,
                    performancePreset,
                    beatmapDataLoader,
                    useTestNoteCutSoundEffects,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Finish(
        &mut self,
        resultsData: *mut MultiplayerResultsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", (resultsData))?;
        Ok(__cordl_ret)
    }
    pub fn get_gameMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_gameMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut ColorScheme> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ColorScheme = __cordl_object
            .invoke("get_colorScheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishWithDisconnect(
        &mut self,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishWithDisconnect", (disconnectedReason))?;
        Ok(__cordl_ret)
    }
    pub fn InitAndSetupScenes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitAndSetupScenes", ())?;
        Ok(__cordl_ret)
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
    pub fn set_colorScheme(
        &mut self,
        value: *mut ColorScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorScheme", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapKey(&mut self) -> quest_hook::libil2cpp::Result<BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapKey = __cordl_object.invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_usingOverrideColorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_usingOverrideColorScheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerLevelScenesTransitionSetupDataSO,
            *mut MultiplayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didDisconnectEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerLevelScenesTransitionSetupDataSO,
            DisconnectedReason,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didDisconnectEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetOrLoadMultiplayerEnvironmentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut EnvironmentInfoSO = __cordl_object
            .invoke("GetOrLoadMultiplayerEnvironmentInfo", ())?;
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
#[cfg(feature = "MultiplayerLevelScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLevelScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
