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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RecordingToolConfigurationProcessor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RecordingToolConfigurationProcessor";
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RecordingSettings,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MenuDestination,
                        >,
                        2usize,
                    >("CreateMenuDestinationFromConfiguration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateMenuDestinationFromConfiguration",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuDestination,
        > = unsafe {
            method.invoke_unchecked(self, (recordingSettings, quitAppAfterRun))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::RecordingSettings,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ObjectsMovementRecorder_InitData,
                        >,
                        1usize,
                    >("CreateObjectsMovementRecorderInitDataFromConfiguration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "CreateObjectsMovementRecorderInitDataFromConfiguration",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObjectsMovementRecorder_InitData,
        > = unsafe { method.invoke_unchecked(self, (recordingSettings))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCharacteristicCollection,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::RecordingToolSettings,
                        >,
                        2usize,
                    >("CreateRecordingToolSettingsFromConfiguration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "CreateRecordingToolSettingsFromConfiguration", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolSettings,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (recordingToolConfiguration, beatmapCharacteristicCollection),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
                        >,
                        1usize,
                    >("DeserializeConfigurationFile")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "DeserializeConfigurationFile", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
        > = unsafe { method.invoke_unchecked(self, (jsonData))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetConfigFilePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("GetConfigFilePath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetConfigFilePath", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultMrcLayersMask() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::LayerMask,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::LayerMask,
                        0usize,
                    >("GetDefaultMrcLayersMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetDefaultMrcLayersMask", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::LayerMask = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerMask(
        &mut self,
        layerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::LayerMask,
                        1usize,
                    >("GetLayerMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetLayerMask", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::LayerMask = unsafe {
            method.invoke_unchecked(self, (layerName))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        >),
                        crate::UnityEngine::LayerMask,
                        1usize,
                    >("GetLayersMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetLayersMask", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::LayerMask = unsafe {
            method.invoke_unchecked(self, (layerNames))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelsModel,
                            >,
                        ),
                        crate::System::ValueTuple_2<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelPack,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                        >,
                        3usize,
                    >("GetLevelPackAndLevelPreviewForLevelId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetLevelPackAndLevelPreviewForLevelId",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        > = unsafe {
            method.invoke_unchecked((), (packId, levelId, beatmapLevelsModel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsRecordingToolEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), bool, 0usize>("IsRecordingToolEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsRecordingToolEnabled", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
                        >,
                        1usize,
                    >("LoadConfiguration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadConfiguration", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration,
        > = unsafe { method.invoke_unchecked(self, (filePath))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadConfigurationFile(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("LoadConfigurationFile")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadConfigurationFile", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (filePath))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatSaberLogger,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IPosesSerializer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::EnvironmentsListModel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelsModel,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        commandLineParserResult,
                        logger,
                        posesSerializer,
                        environmentListModel,
                        beatmapLevelsModel,
                    ),
                )?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_ColorSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RecordingToolConfigurationProcessor/ColorSaveData";
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_PlaybackScreenshot {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RecordingToolConfigurationProcessor/PlaybackScreenshot";
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingConfiguration {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RecordingToolConfigurationProcessor/RecordingConfiguration";
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::RecordingToolConfigurationProcessor_RecordingToolConfiguration {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RecordingToolConfigurationProcessor/RecordingToolConfiguration";
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
