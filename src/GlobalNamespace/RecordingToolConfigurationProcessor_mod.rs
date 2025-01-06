#[cfg(feature = "RecordingToolConfigurationProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolConfigurationProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _commandLineParserResult: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
    pub _logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
    pub _posesSerializer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IPosesSerializer,
    >,
    pub _environmentListModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentsListModel,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
}
#[cfg(feature = "RecordingToolConfigurationProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RecordingToolConfigurationProcessor => ""
    ."RecordingToolConfigurationProcessor"
);
#[cfg(feature = "RecordingToolConfigurationProcessor")]
impl std::ops::Deref for crate::GlobalNamespace::RecordingToolConfigurationProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor")]
impl std::ops::DerefMut for crate::GlobalNamespace::RecordingToolConfigurationProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor")]
impl crate::GlobalNamespace::RecordingToolConfigurationProcessor {
    pub const kDefaultMrcLayersMask: &'static str = "DefaultMrcLayers";
    pub const kEverythingLayerMask: &'static str = "Everything";
    pub const kNormalEnvironmentNameSuffix: &'static str = "Environment";
    pub const kNormalEnvironmentType: &'static str = "Normal";
    pub const kNothingLayerMask: &'static str = "Nothing";
    pub const kSoloMode: &'static str = "Solo";
    #[cfg(feature = "RecordingToolConfigurationProcessor+ColorSaveData")]
    pub type ColorSaveData = crate::GlobalNamespace::RecordingToolConfigurationProcessor_ColorSaveData;
    #[cfg(feature = "RecordingToolConfigurationProcessor+PlaybackScreenshot")]
    pub type PlaybackScreenshot = crate::GlobalNamespace::RecordingToolConfigurationProcessor_PlaybackScreenshot;
    #[cfg(feature = "RecordingToolConfigurationProcessor+RecordingConfiguration")]
    pub type RecordingConfiguration = crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingConfiguration;
    #[cfg(feature = "RecordingToolConfigurationProcessor+RecordingToolConfiguration")]
    pub type RecordingToolConfiguration = crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration;
    pub fn CreateMenuDestinationFromConfiguration(
        &mut self,
        recordingSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingSettings,
        >,
        quitAppAfterRun: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuDestination>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuDestination,
        > = __cordl_object
            .invoke(
                "CreateMenuDestinationFromConfiguration",
                (recordingSettings, quitAppAfterRun),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateObjectsMovementRecorderInitDataFromConfiguration(
        &mut self,
        recordingSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObjectsMovementRecorder_InitData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObjectsMovementRecorder_InitData,
        > = __cordl_object
            .invoke(
                "CreateObjectsMovementRecorderInitDataFromConfiguration",
                (recordingSettings),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateRecordingToolSettingsFromConfiguration(
        &mut self,
        recordingToolConfiguration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
        >,
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::RecordingToolSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolSettings,
        > = __cordl_object
            .invoke(
                "CreateRecordingToolSettingsFromConfiguration",
                (recordingToolConfiguration, beatmapCharacteristicCollection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeConfigurationFile(
        &mut self,
        jsonData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
        > = __cordl_object.invoke("DeserializeConfigurationFile", (jsonData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConfigFilePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetConfigFilePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultMrcLayersMask() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::LayerMask,
    > {
        let __cordl_ret: crate::UnityEngine::LayerMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultMrcLayersMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerMask(
        &mut self,
        layerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("GetLayerMask", (layerName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayersMask(
        &mut self,
        layerNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("GetLayersMask", (layerNames))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelPackAndLevelPreviewForLevelId(
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        >,
    > {
        let __cordl_ret: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetLevelPackAndLevelPreviewForLevelId",
                (packId, levelId, beatmapLevelsModel),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRecordingToolEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsRecordingToolEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadConfiguration(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
        > = __cordl_object.invoke("LoadConfiguration", (filePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadConfigurationFile(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("LoadConfigurationFile", (filePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        commandLineParserResult: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
        posesSerializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPosesSerializer,
        >,
        environmentListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        commandLineParserResult: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
        posesSerializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPosesSerializer,
        >,
        environmentListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingToolConfigurationProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RecordingToolConfigurationProcessor+ColorSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolConfigurationProcessor_ColorSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub includedLayers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub excludedLayers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub backgroundColor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RecordingToolConfigurationProcessor_ColorSaveData,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub packID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub characteristic: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub runLevel: bool,
    pub recordPerformance: bool,
    pub recordingMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub recordingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cameraView: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub addDateTimeSuffixToRecordingName: bool,
    pub screenshotRecording: bool,
    pub screenshotWidth: i32,
    pub screenshotHeight: i32,
    pub framerate: i32,
    pub playbackScreenshots: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::RecordingToolConfigurationProcessor_PlaybackScreenshot,
            >,
        >,
    >,
    pub practice: bool,
    pub startSongTime: f32,
    pub songSpeedMultiplier: f32,
    pub overrideEnvironments: bool,
    pub environmentType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub recordingConfigurations: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingConfiguration,
            >,
        >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
