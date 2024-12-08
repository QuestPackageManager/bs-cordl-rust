#[cfg(feature = "MultiplayerLevelLoader+MultiplayerBeatmapLoaderState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerLevelLoader_MultiplayerBeatmapLoaderState {
    LoadingBeatmap = 1i32,
    NotLoading = 0i32,
    WaitingForCountdown = 2i32,
}
#[cfg(feature = "MultiplayerLevelLoader+MultiplayerBeatmapLoaderState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLevelLoader_MultiplayerBeatmapLoaderState => ""
    ."MultiplayerLevelLoader/MultiplayerBeatmapLoaderState"
);
#[cfg(feature = "MultiplayerLevelLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLevelLoader {
    __cordl_parent: crate::System::Object,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub _beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
    pub stillDownloadingSongEvent: *mut crate::System::Action,
    pub countdownFinishedEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::ILevelGameplaySetupData,
        *mut crate::GlobalNamespace::IBeatmapLevelData,
    >,
    pub _loaderState: crate::GlobalNamespace::MultiplayerLevelLoader_MultiplayerBeatmapLoaderState,
    pub _getBeatmapCancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _getBeatmapLevelResultTask: *mut crate::System::Threading::Tasks::Task_1<
        crate::GlobalNamespace::LoadBeatmapLevelDataResult,
    >,
    pub _gameplaySetupData: *mut crate::GlobalNamespace::ILevelGameplaySetupData,
    pub _beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
    pub _startTime: i64,
    pub _stillDownloadingCalled: bool,
}
#[cfg(feature = "MultiplayerLevelLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerLevelLoader => ""
    ."MultiplayerLevelLoader"
);
#[cfg(feature = "MultiplayerLevelLoader")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLevelLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLevelLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelLoader")]
impl crate::GlobalNamespace::MultiplayerLevelLoader {
    #[cfg(feature = "MultiplayerLevelLoader+MultiplayerBeatmapLoaderState")]
    pub type MultiplayerBeatmapLoaderState = crate::GlobalNamespace::MultiplayerLevelLoader_MultiplayerBeatmapLoaderState;
    #[cfg(feature = "MultiplayerLevelLoader+_LoadBeatmapLevelDataAsync_d__21")]
    pub type _LoadBeatmapLevelDataAsync_d__21 = crate::GlobalNamespace::MultiplayerLevelLoader__LoadBeatmapLevelDataAsync_d__21;
    pub fn ClearLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLoading", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadBeatmapLevelDataAsync(
        &mut self,
        gameplaySetupData: *mut crate::GlobalNamespace::ILevelGameplaySetupData,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::LoadBeatmapLevelDataResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::LoadBeatmapLevelDataResult,
        > = __cordl_object.invoke("LoadBeatmapLevelDataAsync", (gameplaySetupData))?;
        Ok(__cordl_ret)
    }
    pub fn LoadLevel(
        &mut self,
        gameplaySetupData: *mut crate::GlobalNamespace::ILevelGameplaySetupData,
        initialStartTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadLevel", (gameplaySetupData, initialStartTime))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetNewStartTime(
        &mut self,
        newStartTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNewStartTime", (newStartTime))?;
        Ok(__cordl_ret)
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
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
    pub fn add_countdownFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::ILevelGameplaySetupData,
            *mut crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_countdownFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_stillDownloadingSongEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_stillDownloadingSongEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_countdownFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::ILevelGameplaySetupData,
            *mut crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_countdownFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_stillDownloadingSongEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_stillDownloadingSongEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerLevelLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLevelLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
