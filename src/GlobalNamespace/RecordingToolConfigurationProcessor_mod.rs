#[cfg(feature = "RecordingToolConfigurationProcessor+ColorSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolConfigurationProcessor_ColorSaveData {
    __cordl_parent: crate::System::Object,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
#[cfg(feature = "RecordingToolConfigurationProcessor+ColorSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RecordingToolConfigurationProcessor_ColorSaveData => ""
    ."RecordingToolConfigurationProcessor/ColorSaveData"
);
#[cfg(feature = "RecordingToolConfigurationProcessor+ColorSaveData")]
impl std::ops::Deref
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_ColorSaveData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+ColorSaveData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_ColorSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+ColorSaveData")]
impl crate::GlobalNamespace::RecordingToolConfigurationProcessor_ColorSaveData {
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
#[cfg(feature = "RecordingToolConfigurationProcessor+ColorSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_ColorSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+PlaybackScreenshot")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolConfigurationProcessor_PlaybackScreenshot {
    __cordl_parent: crate::System::Object,
    pub name: *mut crate::System::String,
    pub _cordl_type: *mut crate::System::String,
    pub includedLayers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub excludedLayers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub backgroundColor: *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor_ColorSaveData,
}
#[cfg(feature = "RecordingToolConfigurationProcessor+PlaybackScreenshot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RecordingToolConfigurationProcessor_PlaybackScreenshot => ""
    ."RecordingToolConfigurationProcessor/PlaybackScreenshot"
);
#[cfg(feature = "RecordingToolConfigurationProcessor+PlaybackScreenshot")]
impl std::ops::Deref
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_PlaybackScreenshot {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+PlaybackScreenshot")]
impl std::ops::DerefMut
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_PlaybackScreenshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+PlaybackScreenshot")]
impl crate::GlobalNamespace::RecordingToolConfigurationProcessor_PlaybackScreenshot {
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
#[cfg(feature = "RecordingToolConfigurationProcessor+PlaybackScreenshot")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_PlaybackScreenshot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingConfiguration")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolConfigurationProcessor_RecordingConfiguration {
    __cordl_parent: crate::System::Object,
    pub mode: *mut crate::System::String,
    pub packID: *mut crate::System::String,
    pub levelID: *mut crate::System::String,
    pub difficulty: *mut crate::System::String,
    pub characteristic: *mut crate::System::String,
    pub runLevel: bool,
    pub recordPerformance: bool,
    pub recordingMode: *mut crate::System::String,
    pub recordingPath: *mut crate::System::String,
    pub cameraView: *mut crate::System::String,
    pub addDateTimeSuffixToRecordingName: bool,
    pub screenshotRecording: bool,
    pub screenshotWidth: i32,
    pub screenshotHeight: i32,
    pub framerate: i32,
    pub playbackScreenshots: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor_PlaybackScreenshot,
    >,
    pub practice: bool,
    pub startSongTime: f32,
    pub songSpeedMultiplier: f32,
    pub overrideEnvironments: bool,
    pub environmentType: *mut crate::System::String,
    pub environmentName: *mut crate::System::String,
    pub saveToOldFormat: bool,
}
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingConfiguration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingConfiguration => ""
    ."RecordingToolConfigurationProcessor/RecordingConfiguration"
);
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingConfiguration")]
impl std::ops::Deref
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingConfiguration {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingConfiguration")]
impl std::ops::DerefMut
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingConfiguration")]
impl crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingConfiguration {
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
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingConfiguration")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingConfiguration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingToolConfiguration")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolConfigurationProcessor_RecordingToolConfiguration {
    __cordl_parent: crate::System::Object,
    pub recordingConfigurations: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingConfiguration,
    >,
}
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingToolConfiguration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration =>
    ""."RecordingToolConfigurationProcessor/RecordingToolConfiguration"
);
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingToolConfiguration")]
impl std::ops::Deref
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingToolConfiguration")]
impl std::ops::DerefMut
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingToolConfiguration")]
impl crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration {
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
#[cfg(feature = "RecordingToolConfigurationProcessor+RecordingToolConfiguration")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolConfigurationProcessor {
    __cordl_parent: crate::System::Object,
    pub _commandLineParserResult: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
    pub _logger: *mut IBeatSaberLogger,
    pub _posesSerializer: *mut IPosesSerializer,
    pub _environmentListModel: *mut EnvironmentsListModel,
    pub _beatmapLevelsModel: *mut BeatmapLevelsModel,
}
#[cfg(feature = "RecordingToolConfigurationProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for RecordingToolConfigurationProcessor => ""
    ."RecordingToolConfigurationProcessor"
);
#[cfg(feature = "RecordingToolConfigurationProcessor")]
impl std::ops::Deref for RecordingToolConfigurationProcessor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor")]
impl std::ops::DerefMut for RecordingToolConfigurationProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor")]
impl RecordingToolConfigurationProcessor {
    pub const kDefaultMrcLayersMask: &'static str = "DefaultMrcLayers";
    pub const kEverythingLayerMask: &'static str = "Everything";
    pub const kNormalEnvironmentNameSuffix: &'static str = "Environment";
    pub const kNormalEnvironmentType: &'static str = "Normal";
    pub const kNothingLayerMask: &'static str = "Nothing";
    pub const kSoloMode: &'static str = "Solo";
    #[cfg(feature = "RecordingToolConfigurationProcessor+RecordingToolConfiguration")]
    pub type RecordingToolConfiguration = crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration;
    #[cfg(feature = "RecordingToolConfigurationProcessor+PlaybackScreenshot")]
    pub type PlaybackScreenshot = crate::GlobalNamespace::RecordingToolConfigurationProcessor_PlaybackScreenshot;
    #[cfg(feature = "RecordingToolConfigurationProcessor+RecordingConfiguration")]
    pub type RecordingConfiguration = crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingConfiguration;
    #[cfg(feature = "RecordingToolConfigurationProcessor+ColorSaveData")]
    pub type ColorSaveData = crate::GlobalNamespace::RecordingToolConfigurationProcessor_ColorSaveData;
    pub fn CreateMenuDestinationFromConfiguration(
        &mut self,
        recordingSettings: *mut RecordingSettings,
        quitAppAfterRun: bool,
    ) -> quest_hook::libil2cpp::Result<*mut MenuDestination> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MenuDestination = __cordl_object
            .invoke(
                "CreateMenuDestinationFromConfiguration",
                (recordingSettings, quitAppAfterRun),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateObjectsMovementRecorderInitDataFromConfiguration(
        &mut self,
        recordingSettings: *mut RecordingSettings,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ObjectsMovementRecorder_InitData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ObjectsMovementRecorder_InitData = __cordl_object
            .invoke(
                "CreateObjectsMovementRecorderInitDataFromConfiguration",
                (recordingSettings),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateRecordingToolSettingsFromConfiguration(
        &mut self,
        recordingToolConfiguration: *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
        beatmapCharacteristicCollection: *mut BeatmapCharacteristicCollection,
    ) -> quest_hook::libil2cpp::Result<*mut RecordingToolSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut RecordingToolSettings = __cordl_object
            .invoke(
                "CreateRecordingToolSettingsFromConfiguration",
                (recordingToolConfiguration, beatmapCharacteristicCollection),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeConfigurationFile(
        &mut self,
        jsonData: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration = __cordl_object
            .invoke("DeserializeConfigurationFile", (jsonData))?;
        Ok(__cordl_ret)
    }
    pub fn GetConfigFilePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetConfigFilePath", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLayerMask(
        &mut self,
        layerName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("GetLayerMask", (layerName))?;
        Ok(__cordl_ret)
    }
    pub fn GetLayersMask(
        &mut self,
        layerNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("GetLayersMask", (layerNames))?;
        Ok(__cordl_ret)
    }
    pub fn IsRecordingToolEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsRecordingToolEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadConfiguration(
        &mut self,
        filePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration = __cordl_object
            .invoke("LoadConfiguration", (filePath))?;
        Ok(__cordl_ret)
    }
    pub fn LoadConfigurationFile(
        &mut self,
        filePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LoadConfigurationFile", (filePath))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        commandLineParserResult: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
        logger: *mut IBeatSaberLogger,
        posesSerializer: *mut IPosesSerializer,
        environmentListModel: *mut EnvironmentsListModel,
        beatmapLevelsModel: *mut BeatmapLevelsModel,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    commandLineParserResult,
                    logger,
                    posesSerializer,
                    environmentListModel,
                    beatmapLevelsModel,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        commandLineParserResult: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
        logger: *mut IBeatSaberLogger,
        posesSerializer: *mut IPosesSerializer,
        environmentListModel: *mut EnvironmentsListModel,
        beatmapLevelsModel: *mut BeatmapLevelsModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    commandLineParserResult,
                    logger,
                    posesSerializer,
                    environmentListModel,
                    beatmapLevelsModel,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor")]
impl quest_hook::libil2cpp::ObjectType for RecordingToolConfigurationProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
