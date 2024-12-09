#[cfg(feature = "PerformanceConfigurationChecks")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformanceConfigurationChecks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub appConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig,
    pub xrConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig,
    pub ovrConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig,
    pub oculusXrConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig,
    pub gameConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_GameConfig,
    pub playerConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig,
    pub levelConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig,
    pub invalid: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch,
    >,
}
#[cfg(feature = "PerformanceConfigurationChecks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PerformanceConfigurationChecks
    => ""."PerformanceConfigurationChecks"
);
#[cfg(feature = "PerformanceConfigurationChecks")]
impl std::ops::Deref for crate::GlobalNamespace::PerformanceConfigurationChecks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks")]
impl std::ops::DerefMut for crate::GlobalNamespace::PerformanceConfigurationChecks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks {
    #[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
    pub type AppConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+GameConfig")]
    pub type GameConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_GameConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
    pub type LevelConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
    pub type Mismatch = crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch;
    #[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
    pub type OVRConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
    pub type OculusXRConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
    pub type PlayerConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig;
    #[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
    pub type XRConfig = crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig;
    pub fn CreateErrorLog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("CreateErrorLog", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetExpected(
        &mut self,
        mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
        graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
        playerSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetExpected",
                (mainSettingsHandler, graphicSettingsHandler, playerSettings, modifiers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn VerifyEntry<T>(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        expected: T,
        actual: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("VerifyEntry", (name, expected, actual))?;
        Ok(__cordl_ret)
    }
    pub fn VerifyExpected(
        &mut self,
        mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
        graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
        playerSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "VerifyExpected",
                (mainSettingsHandler, graphicSettingsHandler, playerSettings, modifiers),
            )?;
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
}
#[cfg(feature = "PerformanceConfigurationChecks")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PerformanceConfigurationChecks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationChecks_AppConfig {
    pub targetFrameRate: i32,
    pub systemLanguage: crate::UnityEngine::SystemLanguage,
    pub runInBackground: bool,
    pub backgroundLoadingPriority: crate::UnityEngine::ThreadPriority,
}
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationChecks_AppConfig => ""
    ."PerformanceConfigurationChecks/AppConfig"
);
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+AppConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+GameConfig")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationChecks_GameConfig {
    pub roomCenter: crate::UnityEngine::Vector3,
    pub roomRotation: f64,
    pub controllerPosition: crate::UnityEngine::Vector3,
    pub controllerRotation: crate::UnityEngine::Vector3,
    pub hapticFeedback: bool,
    pub smoothCameraEnabled: bool,
    pub smoothCameraFieldOfView: f64,
    pub smoothCameraPositionSmooth: f64,
    pub smoothCameraRotationSmooth: f64,
    pub smoothCameraThirdPersonEnabled: bool,
    pub smoothCameraThirdPersonPosition: crate::UnityEngine::Vector3,
    pub smoothCameraThirdPersonEulerAngles: crate::UnityEngine::Vector3,
    pub volume: f64,
    pub ambientVolumeScale: f64,
    pub overrideAudioLatency: bool,
    pub audioLatency: f64,
    pub enableFPSCounter: bool,
    pub enableFPSRecorder: bool,
    pub enableMemoryTracker: bool,
    pub createScreenshotDuringTheGame: bool,
    pub language: crate::BGLib::Polyglot::Language,
    pub pauseButtonPressDurationLevel: i32,
    pub useCustomServerEnvironment: bool,
    pub forceGameLiftServerEnvironment: bool,
    pub customServerHostName: *mut quest_hook::libil2cpp::Il2CppString,
    pub windowResolution: crate::UnityEngine::Vector2Int,
    pub windowMode: crate::BeatSaber::GameSettings::WindowMode,
    pub performancePresetKey: *mut quest_hook::libil2cpp::Il2CppString,
    pub renderViewportScale: f64,
    pub targetFramerate: f64,
    pub vSyncCount: i32,
    pub maxQueuedFrames: i32,
    pub vrResolutionScale: f64,
    pub menuVRResolutionScaleMultiplier: f64,
    pub antiAliasingLevel: i32,
    pub bloomPrePassTextureEffect: crate::BeatSaber::PerformancePresets::BloomPrepassTextureEffectPreset,
    pub mainEffectGraphicsSettings: crate::BeatSaber::PerformancePresets::MainEffectPreset,
    pub mirrorGraphicsSettings: crate::BeatSaber::PerformancePresets::MirrorQualityPreset,
    pub maxNumberOfCutSoundEffects: i32,
    pub maxShockwaveParticles: i32,
    pub burnMarkTrailsEnabled: bool,
    pub smokeGraphicsSettings: bool,
    pub screenDisplacementEffectsEnabled: bool,
    pub obstaclesQuality: crate::BeatSaber::PerformancePresets::ObstaclesQuality,
    pub cpuLevel: i32,
    pub gpuLevel: i32,
}
#[cfg(feature = "PerformanceConfigurationChecks+GameConfig")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationChecks_GameConfig => ""
    ."PerformanceConfigurationChecks/GameConfig"
);
#[cfg(feature = "PerformanceConfigurationChecks+GameConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_GameConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+GameConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_GameConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationChecks_LevelConfig {
    pub modifiers: crate::GlobalNamespace::GameplayModifierMask,
}
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig => ""
    ."PerformanceConfigurationChecks/LevelConfig"
);
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationChecks_Mismatch {
    pub message: *mut quest_hook::libil2cpp::Il2CppString,
    pub frames: i32,
}
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationChecks_Mismatch => ""
    ."PerformanceConfigurationChecks/Mismatch"
);
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+Mismatch")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch {}
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationChecks_OVRConfig {
    pub spaceWarpEnabled: bool,
    pub chromatic: bool,
    pub monoscopic: bool,
    pub colorGamut: crate::GlobalNamespace::OVRManager_ColorSpace,
    pub nativeColorGamut: crate::GlobalNamespace::OVRManager_ColorSpace,
    pub compositionMethod: crate::GlobalNamespace::OVRManager_CompositionMethod,
    pub enableMixedReality: bool,
    pub trackingOriginType: crate::GlobalNamespace::OVRManager_TrackingOrigin,
    pub usePositionTracking: bool,
    pub useRotationTracking: bool,
    pub sharpenType: crate::GlobalNamespace::OVRPlugin_LayerSharpenType,
    pub enableDynamicResolution: bool,
    pub minDynamicResolutionScale: f32,
    pub maxDynamicResolutionScale: f32,
    pub simultaneousHandsAndControllersSupport: bool,
    pub suggestedCpuPerfLevel: crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
    pub suggestedGpuPerfLevel: crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
    pub systemDisplayFrequency: f32,
    pub eyeTrackedFoveatedRenderingEnabled: bool,
    pub foveatedRenderingLevel: crate::GlobalNamespace::OVRPlugin_FoveatedRenderingLevel,
    pub useDynamicFoveatedRendering: bool,
    pub gpuUtilSupported: bool,
    pub gpuUtilLevel: f32,
    pub eyeFovPremultipliedAlphaModeEnabled: bool,
    pub asymmetricFovEnabled: bool,
    pub eyeTextureArrayEnabled: bool,
    pub localDimmingSupported: bool,
    pub localDimming: bool,
}
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig => ""
    ."PerformanceConfigurationChecks/OVRConfig"
);
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+OVRConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationChecks_OculusXRConfig {
    pub boundaryDimension: crate::UnityEngine::Vector3,
}
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig => ""
    ."PerformanceConfigurationChecks/OculusXRConfig"
);
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+OculusXRConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationChecks_PlayerConfig {
    pub leftHanded: bool,
    pub automaticPlayerHeight: bool,
    pub playerHeight: f32,
    pub noteJumpDurationTypeSettings: crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    pub noteJumpFixedDuration: f32,
    pub noteJumpStartBeatOffset: f32,
    pub autoRestart: bool,
    pub headsetHapticIntensity: f32,
    pub arcsHapticFeedback: bool,
    pub reduceDebris: bool,
    pub noFailEffects: bool,
    pub hideNoteSpawnEffect: bool,
    pub arcVisibility: crate::GlobalNamespace::ArcVisibilityType,
    pub saberTrailIntensity: f32,
    pub noTextsAndHuds: bool,
    pub advancedHud: bool,
    pub sfxVolume: f32,
    pub adaptiveSfx: bool,
    pub environmentEffectsFilterDefaultPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    pub environmentEffectsFilterExpertPlusPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
}
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig => ""
    ."PerformanceConfigurationChecks/PlayerConfig"
);
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+PlayerConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig {}
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceConfigurationChecks_XRConfig {
    pub enabled: bool,
    pub isDeviceActive: bool,
    pub loadedDeviceName: *mut quest_hook::libil2cpp::Il2CppString,
    pub stereoRenderingMode: crate::UnityEngine::XR::XRSettings_StereoRenderingMode,
    pub eyeTextureWidth: i32,
    pub eyeTextureHeight: i32,
    pub eyeTextureResolutionScale: f32,
    pub deviceEyeTextureDimension: crate::UnityEngine::Rendering::TextureDimension,
    pub renderViewportScale: f32,
    pub occlusionMaskScale: f32,
    pub useOcclusionMesh: bool,
}
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceConfigurationChecks_XRConfig => ""
    ."PerformanceConfigurationChecks/XRConfig"
);
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceConfigurationChecks+XRConfig")]
impl crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig {}
