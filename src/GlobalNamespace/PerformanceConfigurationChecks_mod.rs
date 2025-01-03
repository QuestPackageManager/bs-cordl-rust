#[cfg(feature = "PerformanceConfigurationChecks")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformanceConfigurationChecks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub appConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_AppConfig,
    pub xrConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_XRConfig,
    pub ovrConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_OVRConfig,
    pub oculusXrConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_OculusXRConfig,
    pub settingsConfig: crate::BeatSaber::Settings::Settings,
    pub playerConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_PlayerConfig,
    pub levelConfig: crate::GlobalNamespace::PerformanceConfigurationChecks_LevelConfig,
    pub invalid: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            crate::GlobalNamespace::PerformanceConfigurationChecks_Mismatch,
        >,
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("CreateErrorLog", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetExpected(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        playerSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        recPlayState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecPlayBehaviour_State,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetExpected", (settings, playerSettings, modifiers, recPlayState))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyEntry<T>(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn VerifyExpected(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        playerSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        recPlayState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecPlayBehaviour_State,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "VerifyExpected",
                (settings, playerSettings, modifiers, recPlayState),
            )?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Default)]
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
#[cfg(feature = "PerformanceConfigurationChecks+LevelConfig")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
pub struct PerformanceConfigurationChecks_Mismatch {
    pub message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
pub struct PerformanceConfigurationChecks_XRConfig {
    pub enabled: bool,
    pub isDeviceActive: bool,
    pub loadedDeviceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
