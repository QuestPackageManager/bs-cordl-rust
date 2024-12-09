#[cfg(feature = "RecordingToolManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameplayLevelSceneTransitionEvents: *mut crate::GlobalNamespace::GameplayLevelSceneTransitionEvents,
    pub _gameScenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    pub _recordingToolEnabled: bool,
    pub _performanceRecordingEnabled: bool,
    pub _configJsonData: *mut quest_hook::libil2cpp::Il2CppString,
    pub _recordingToolSettings: *mut crate::GlobalNamespace::RecordingToolSettings,
    pub _configurationProcessor: *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _logger: *mut crate::GlobalNamespace::IBeatSaberLogger,
    pub _posesSerializer: *mut crate::GlobalNamespace::IPosesSerializer,
    pub _currentRecordingIndex: i32,
}
#[cfg(feature = "RecordingToolManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RecordingToolManager => ""
    ."RecordingToolManager"
);
#[cfg(feature = "RecordingToolManager")]
impl std::ops::Deref for crate::GlobalNamespace::RecordingToolManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::RecordingToolManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolManager")]
impl crate::GlobalNamespace::RecordingToolManager {
    pub const kRecordingToolId: &'static str = "RecordingTool";
    #[cfg(feature = "RecordingToolManager+SetupData")]
    pub type SetupData = crate::GlobalNamespace::RecordingToolManager_SetupData;
    #[cfg(
        feature = "RecordingToolManager+_HandleGameplayLevelSceneTransitionEventsAnyGameplayLevelDidFinish_d__28"
    )]
    pub type _HandleGameplayLevelSceneTransitionEventsAnyGameplayLevelDidFinish_d__28 = crate::GlobalNamespace::RecordingToolManager__HandleGameplayLevelSceneTransitionEventsAnyGameplayLevelDidFinish_d__28;
    #[cfg(feature = "RecordingToolManager+__c")]
    pub type __c = crate::GlobalNamespace::RecordingToolManager___c;
    pub fn BindNextRecording(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        recordingSettings: *mut crate::GlobalNamespace::RecordingSettings,
        quitAppAfterRun: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::MenuDestination> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MenuDestination = __cordl_object
            .invoke(
                "BindNextRecording",
                (container, recordingSettings, quitAppAfterRun),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleGameplayLevelSceneTransitionEventsAnyGameplayLevelDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleGameplayLevelSceneTransitionEventsAnyGameplayLevelDidFinish",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        processor: *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor,
        beatmapCharacteristicCollection: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
        diContainer: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (processor, beatmapCharacteristicCollection, diContainer),
            )?;
        Ok(__cordl_object)
    }
    pub fn StartNextRecording(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartNextRecording", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        processor: *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor,
        beatmapCharacteristicCollection: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
        diContainer: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (processor, beatmapCharacteristicCollection, diContainer))?;
        Ok(__cordl_ret)
    }
    pub fn get_configJsonData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_configJsonData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_performanceRecordingEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_performanceRecordingEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_posesSerializer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::IPosesSerializer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IPosesSerializer = __cordl_object
            .invoke("get_posesSerializer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_recordingToolEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_recordingToolEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_recordingToolSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::RecordingToolSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::RecordingToolSettings = __cordl_object
            .invoke("get_recordingToolSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showRecordingToolScene(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showRecordingToolScene", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RecordingToolManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RecordingToolManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RecordingToolManager+SetupData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RecordingToolManager_SetupData {
    pub profileSong: bool,
    pub runAutopilot: bool,
}
#[cfg(feature = "RecordingToolManager+SetupData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RecordingToolManager_SetupData
    => ""."RecordingToolManager/SetupData"
);
#[cfg(feature = "RecordingToolManager+SetupData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::RecordingToolManager_SetupData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "RecordingToolManager+SetupData")]
impl crate::GlobalNamespace::RecordingToolManager_SetupData {}
