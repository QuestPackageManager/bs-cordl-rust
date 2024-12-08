#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePreset")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformancePreset {
    __cordl_parent: crate::System::Object,
    pub _dataVersion: u32,
    pub _presetName: *mut crate::System::String,
    pub _presetNameLocalizationTag: *mut crate::System::String,
    pub _renderViewportScale: f32,
    pub _vrResolutionScale: f32,
    pub _menuVRResolutionScaleMultiplier: f32,
    pub _antiAliasingLevel: i32,
    pub _targetFramerate: f32,
    pub _vSyncCount: i32,
    pub _maxQueuedFrames: i32,
    pub _mirrorGraphics: crate::BeatSaber::PerformancePresets::MirrorQualityPreset,
    pub _mainEffectGraphics: crate::BeatSaber::PerformancePresets::MainEffectPreset,
    pub _bloomPrePassTextureEffect: crate::BeatSaber::PerformancePresets::BloomPrepassTextureEffectPreset,
    pub _smokeGraphics: bool,
    pub _depthTexture: bool,
    pub _burnMarkTrails: bool,
    pub _screenDisplacementEffects: bool,
    pub _maxShockwaveParticles: i32,
    pub _maxNumberOfCutSoundEffects: i32,
    pub _defaultAudioLatency: f32,
    pub _obstaclesQuality: crate::BeatSaber::PerformancePresets::ObstaclesQuality,
    pub _questSettings: *mut crate::BeatSaber::PerformancePresets::QuestPerformanceSettings,
    pub _environmentOverrides: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::PerformancePresets::EnvironmentOverride,
    >,
}
#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePreset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::PerformancePresets::PerformancePreset
    => "BeatSaber.PerformancePresets"."PerformancePreset"
);
#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePreset")]
impl std::ops::Deref for crate::BeatSaber::PerformancePresets::PerformancePreset {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePreset")]
impl std::ops::DerefMut for crate::BeatSaber::PerformancePresets::PerformancePreset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePreset")]
impl crate::BeatSaber::PerformancePresets::PerformancePreset {
    pub fn get_burnMarkTrails(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_burnMarkTrails", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_questSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::PerformancePresets::QuestPerformanceSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::PerformancePresets::QuestPerformanceSettings = __cordl_object
            .invoke("get_questSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetFramerate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_targetFramerate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_antiAliasingLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_antiAliasingLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::PerformancePresets::EnvironmentOverride,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::PerformancePresets::EnvironmentOverride,
        > = __cordl_object.invoke("get_environmentOverrides", ())?;
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
    pub fn get_maxQueuedFrames(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxQueuedFrames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_vrResolutionScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_vrResolutionScale", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_renderViewportScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_renderViewportScale", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_depthTexture(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_depthTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_vSyncCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_vSyncCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screenDisplacementEffects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_screenDisplacementEffects", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxShockwaveParticles(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxShockwaveParticles", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxNumberOfCutSoundEffects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_maxNumberOfCutSoundEffects", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_smokeGraphics(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_smokeGraphics", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_presetNameLocalizationTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_presetNameLocalizationTag", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultAudioLatency(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_defaultAudioLatency", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mirrorGraphics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::PerformancePresets::MirrorQualityPreset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::PerformancePresets::MirrorQualityPreset = __cordl_object
            .invoke("get_mirrorGraphics", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mainEffectGraphics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::PerformancePresets::MainEffectPreset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::PerformancePresets::MainEffectPreset = __cordl_object
            .invoke("get_mainEffectGraphics", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_obstaclesQuality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::PerformancePresets::ObstaclesQuality,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::PerformancePresets::ObstaclesQuality = __cordl_object
            .invoke("get_obstaclesQuality", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_presetName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_presetName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dataVersion(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_dataVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bloomPrePassTextureEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::PerformancePresets::BloomPrepassTextureEffectPreset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::PerformancePresets::BloomPrepassTextureEffectPreset = __cordl_object
            .invoke("get_bloomPrePassTextureEffect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_menuVRResolutionScaleMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_menuVRResolutionScaleMultiplier", ())?;
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
#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePreset")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::PerformancePresets::PerformancePreset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
