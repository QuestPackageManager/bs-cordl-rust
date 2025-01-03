#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacySettingsV2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mainSettings: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Settings::LegacySettingsV2_MainSettings,
    >,
    pub graphicSettings: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Settings::LegacySettingsV2_GraphicSettings,
    >,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::LegacySettingsV2 =>
    "BeatSaber.Settings"."LegacySettingsV2"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2")]
impl std::ops::Deref for crate::BeatSaber::Settings::LegacySettingsV2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::LegacySettingsV2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2")]
impl crate::BeatSaber::Settings::LegacySettingsV2 {
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+AudioSettings")]
    pub type AudioSettings = crate::BeatSaber::Settings::LegacySettingsV2_AudioSettings;
    #[cfg(
        feature = "BeatSaber+Settings+LegacySettingsV2+BloomPrepassTextureEffectPreset"
    )]
    pub type BloomPrepassTextureEffectPreset = crate::BeatSaber::Settings::LegacySettingsV2_BloomPrepassTextureEffectPreset;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ControllerSettings")]
    pub type ControllerSettings = crate::BeatSaber::Settings::LegacySettingsV2_ControllerSettings;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+CustomServerSettings")]
    pub type CustomServerSettings = crate::BeatSaber::Settings::LegacySettingsV2_CustomServerSettings;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+DebugSettings")]
    pub type DebugSettings = crate::BeatSaber::Settings::LegacySettingsV2_DebugSettings;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+FoveatedRenderingLevel")]
    pub type FoveatedRenderingLevel = crate::BeatSaber::Settings::LegacySettingsV2_FoveatedRenderingLevel;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+GraphicSettings")]
    pub type GraphicSettings = crate::BeatSaber::Settings::LegacySettingsV2_GraphicSettings;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MainEffectPreset")]
    pub type MainEffectPreset = crate::BeatSaber::Settings::LegacySettingsV2_MainEffectPreset;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MainSettings")]
    pub type MainSettings = crate::BeatSaber::Settings::LegacySettingsV2_MainSettings;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MirrorQualityPreset")]
    pub type MirrorQualityPreset = crate::BeatSaber::Settings::LegacySettingsV2_MirrorQualityPreset;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ObstaclesQuality")]
    pub type ObstaclesQuality = crate::BeatSaber::Settings::LegacySettingsV2_ObstaclesQuality;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+PerformancePreset")]
    pub type PerformancePreset = crate::BeatSaber::Settings::LegacySettingsV2_PerformancePreset;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ProcessorPerformanceLevel")]
    pub type ProcessorPerformanceLevel = crate::BeatSaber::Settings::LegacySettingsV2_ProcessorPerformanceLevel;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+SmoothCameraSettings")]
    pub type SmoothCameraSettings = crate::BeatSaber::Settings::LegacySettingsV2_SmoothCameraSettings;
    #[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+WindowMode")]
    pub type WindowMode = crate::BeatSaber::Settings::LegacySettingsV2_WindowMode;
    pub fn ApplyTo(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyTo", (settings))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Settings::LegacySettingsV2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+AudioSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacySettingsV2_AudioSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub volume: f32,
    pub ambientVolumeScale: f32,
    pub overrideAudioLatency: bool,
    pub audioLatency: f32,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+AudioSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_AudioSettings => "BeatSaber.Settings"
    ."LegacySettingsV2/AudioSettings"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+AudioSettings")]
impl std::ops::Deref for crate::BeatSaber::Settings::LegacySettingsV2_AudioSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+AudioSettings")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::LegacySettingsV2_AudioSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+AudioSettings")]
impl crate::BeatSaber::Settings::LegacySettingsV2_AudioSettings {
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+AudioSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Settings::LegacySettingsV2_AudioSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+BloomPrepassTextureEffectPreset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySettingsV2_BloomPrepassTextureEffectPreset {
    HD = 0i32,
    HDWithoutToneMapping = 1i32,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+BloomPrepassTextureEffectPreset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_BloomPrepassTextureEffectPreset =>
    "BeatSaber.Settings"."LegacySettingsV2/BloomPrepassTextureEffectPreset"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ControllerSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacySettingsV2_ControllerSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub positionOffset: crate::Unity::Mathematics::float3,
    pub rotationOffset: crate::Unity::Mathematics::float3,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ControllerSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_ControllerSettings => "BeatSaber.Settings"
    ."LegacySettingsV2/ControllerSettings"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ControllerSettings")]
impl std::ops::Deref
for crate::BeatSaber::Settings::LegacySettingsV2_ControllerSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ControllerSettings")]
impl std::ops::DerefMut
for crate::BeatSaber::Settings::LegacySettingsV2_ControllerSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ControllerSettings")]
impl crate::BeatSaber::Settings::LegacySettingsV2_ControllerSettings {
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ControllerSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Settings::LegacySettingsV2_ControllerSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+CustomServerSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacySettingsV2_CustomServerSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub useCustomServerEnvironment: bool,
    pub forceGameLiftServerEnvironment: bool,
    pub customServerHostName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+CustomServerSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_CustomServerSettings => "BeatSaber.Settings"
    ."LegacySettingsV2/CustomServerSettings"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+CustomServerSettings")]
impl std::ops::Deref
for crate::BeatSaber::Settings::LegacySettingsV2_CustomServerSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+CustomServerSettings")]
impl std::ops::DerefMut
for crate::BeatSaber::Settings::LegacySettingsV2_CustomServerSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+CustomServerSettings")]
impl crate::BeatSaber::Settings::LegacySettingsV2_CustomServerSettings {
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+CustomServerSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Settings::LegacySettingsV2_CustomServerSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+DebugSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacySettingsV2_DebugSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub enableFpsCounter: bool,
    pub enableFpsRecorder: bool,
    pub enableMemoryTracker: bool,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+DebugSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_DebugSettings => "BeatSaber.Settings"
    ."LegacySettingsV2/DebugSettings"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+DebugSettings")]
impl std::ops::Deref for crate::BeatSaber::Settings::LegacySettingsV2_DebugSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+DebugSettings")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::LegacySettingsV2_DebugSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+DebugSettings")]
impl crate::BeatSaber::Settings::LegacySettingsV2_DebugSettings {
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+DebugSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Settings::LegacySettingsV2_DebugSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+FoveatedRenderingLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySettingsV2_FoveatedRenderingLevel {
    High = 3i32,
    HighTop = 4i32,
    Low = 1i32,
    Medium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+FoveatedRenderingLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_FoveatedRenderingLevel =>
    "BeatSaber.Settings"."LegacySettingsV2/FoveatedRenderingLevel"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+GraphicSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacySettingsV2_GraphicSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub performancePresetKey: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub customPreset: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Settings::LegacySettingsV2_PerformancePreset,
    >,
    pub windowMode: crate::BeatSaber::Settings::LegacySettingsV2_WindowMode,
    pub windowResolution: crate::Unity::Mathematics::int2,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+GraphicSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_GraphicSettings => "BeatSaber.Settings"
    ."LegacySettingsV2/GraphicSettings"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+GraphicSettings")]
impl std::ops::Deref for crate::BeatSaber::Settings::LegacySettingsV2_GraphicSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+GraphicSettings")]
impl std::ops::DerefMut
for crate::BeatSaber::Settings::LegacySettingsV2_GraphicSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+GraphicSettings")]
impl crate::BeatSaber::Settings::LegacySettingsV2_GraphicSettings {
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+GraphicSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Settings::LegacySettingsV2_GraphicSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MainEffectPreset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySettingsV2_MainEffectPreset {
    Off = 0i32,
    Pyramid = 1i32,
    PyramidForBaking = 2i32,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MainEffectPreset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_MainEffectPreset => "BeatSaber.Settings"
    ."LegacySettingsV2/MainEffectPreset"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MainSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacySettingsV2_MainSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub audioSettings: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Settings::LegacySettingsV2_AudioSettings,
    >,
    pub smoothCameraSettings: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Settings::LegacySettingsV2_SmoothCameraSettings,
    >,
    pub controllerSettings: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Settings::LegacySettingsV2_ControllerSettings,
    >,
    pub customServerSettings: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Settings::LegacySettingsV2_CustomServerSettings,
    >,
    pub debugSettings: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Settings::LegacySettingsV2_DebugSettings,
    >,
    pub language: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub pauseButtonPressDurationLevel: i32,
    pub roomCenter: crate::Unity::Mathematics::float3,
    pub roomRotation: f32,
    pub hapticFeedback: bool,
    pub createScreenshotDuringTheGame: bool,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MainSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_MainSettings => "BeatSaber.Settings"
    ."LegacySettingsV2/MainSettings"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MainSettings")]
impl std::ops::Deref for crate::BeatSaber::Settings::LegacySettingsV2_MainSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MainSettings")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::LegacySettingsV2_MainSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MainSettings")]
impl crate::BeatSaber::Settings::LegacySettingsV2_MainSettings {
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MainSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Settings::LegacySettingsV2_MainSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MirrorQualityPreset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySettingsV2_MirrorQualityPreset {
    Fake = 1i32,
    Off = 0i32,
    RenderedHQ = 3i32,
    RenderedLQ = 2i32,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+MirrorQualityPreset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_MirrorQualityPreset => "BeatSaber.Settings"
    ."LegacySettingsV2/MirrorQualityPreset"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ObstaclesQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySettingsV2_ObstaclesQuality {
    DefaultObstacleQualityBaseOnDisplacement = 0i32,
    ObstacleHW = 3i32,
    ObstacleLW = 2i32,
    TexturedObstacle = 1i32,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ObstaclesQuality")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_ObstaclesQuality => "BeatSaber.Settings"
    ."LegacySettingsV2/ObstaclesQuality"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+PerformancePreset")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacySettingsV2_PerformancePreset {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub presetNameLocalizationTag: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub renderViewportScale: f32,
    pub vrResolutionScale: f32,
    pub menuVRResolutionScaleMultiplier: f32,
    pub antiAliasingLevel: i32,
    pub targetFramerate: f32,
    pub vSyncCount: i32,
    pub maxQueuedFrames: i32,
    pub smokeGraphics: bool,
    pub depthTexture: bool,
    pub burnMarkTrails: bool,
    pub screenDisplacementEffects: bool,
    pub maxShockwaveParticles: i32,
    pub maxNumberOfCutSoundEffects: i32,
    pub mirrorGraphics: crate::BeatSaber::Settings::LegacySettingsV2_MirrorQualityPreset,
    pub mainEffectGraphics: crate::BeatSaber::Settings::LegacySettingsV2_MainEffectPreset,
    pub bloomPrePassTextureEffect: crate::BeatSaber::Settings::LegacySettingsV2_BloomPrepassTextureEffectPreset,
    pub obstaclesQuality: crate::BeatSaber::Settings::LegacySettingsV2_ObstaclesQuality,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+PerformancePreset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_PerformancePreset => "BeatSaber.Settings"
    ."LegacySettingsV2/PerformancePreset"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+PerformancePreset")]
impl std::ops::Deref for crate::BeatSaber::Settings::LegacySettingsV2_PerformancePreset {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+PerformancePreset")]
impl std::ops::DerefMut
for crate::BeatSaber::Settings::LegacySettingsV2_PerformancePreset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+PerformancePreset")]
impl crate::BeatSaber::Settings::LegacySettingsV2_PerformancePreset {
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+PerformancePreset")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Settings::LegacySettingsV2_PerformancePreset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ProcessorPerformanceLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySettingsV2_ProcessorPerformanceLevel {
    Boost = 3i32,
    PowerSavings = 0i32,
    SustainedHigh = 2i32,
    SustainedLow = 1i32,
    Unknown = -1i32,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+ProcessorPerformanceLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_ProcessorPerformanceLevel =>
    "BeatSaber.Settings"."LegacySettingsV2/ProcessorPerformanceLevel"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+SmoothCameraSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacySettingsV2_SmoothCameraSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub enabled: bool,
    pub fov: f32,
    pub rotationSmooth: f32,
    pub positionSmooth: f32,
    pub thirdPersonEnabled: bool,
    pub thirdPersonPosition: crate::Unity::Mathematics::float3,
    pub thirdPersonEulerAngles: crate::Unity::Mathematics::float3,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+SmoothCameraSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::LegacySettingsV2_SmoothCameraSettings => "BeatSaber.Settings"
    ."LegacySettingsV2/SmoothCameraSettings"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+SmoothCameraSettings")]
impl std::ops::Deref
for crate::BeatSaber::Settings::LegacySettingsV2_SmoothCameraSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+SmoothCameraSettings")]
impl std::ops::DerefMut
for crate::BeatSaber::Settings::LegacySettingsV2_SmoothCameraSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+SmoothCameraSettings")]
impl crate::BeatSaber::Settings::LegacySettingsV2_SmoothCameraSettings {
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+SmoothCameraSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Settings::LegacySettingsV2_SmoothCameraSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+WindowMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySettingsV2_WindowMode {
    Fullscreen = 1i32,
    Windowed = 0i32,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV2+WindowMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::LegacySettingsV2_WindowMode
    => "BeatSaber.Settings"."LegacySettingsV2/WindowMode"
);
