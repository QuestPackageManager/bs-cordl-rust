#[cfg(feature = "BeatSaber+PerformancePresets+CustomPerformancePreset")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomPerformancePreset {
    __cordl_parent: crate::BeatSaber::PerformancePresets::PerformancePreset,
    pub _isDirty_k__BackingField: bool,
}
#[cfg(feature = "BeatSaber+PerformancePresets+CustomPerformancePreset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::PerformancePresets::CustomPerformancePreset =>
    "BeatSaber.PerformancePresets"."CustomPerformancePreset"
);
#[cfg(feature = "BeatSaber+PerformancePresets+CustomPerformancePreset")]
impl std::ops::Deref for crate::BeatSaber::PerformancePresets::CustomPerformancePreset {
    type Target = crate::BeatSaber::PerformancePresets::PerformancePreset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+CustomPerformancePreset")]
impl std::ops::DerefMut
for crate::BeatSaber::PerformancePresets::CustomPerformancePreset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+CustomPerformancePreset")]
impl crate::BeatSaber::PerformancePresets::CustomPerformancePreset {
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
    pub fn get_antiAliasingLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_antiAliasingLevel", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_burnMarkTrails(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_burnMarkTrails", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultAudioLatency(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_defaultAudioLatency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_depthTexture(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_depthTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDirty", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_maxNumberOfCutSoundEffects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_maxNumberOfCutSoundEffects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxQueuedFrames(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxQueuedFrames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxShockwaveParticles(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxShockwaveParticles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_menuVRResolutionScaleMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_menuVRResolutionScaleMultiplier", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_presetName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_presetName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderViewportScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_renderViewportScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_screenDisplacementEffects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_screenDisplacementEffects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_smokeGraphics(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_smokeGraphics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetFramerate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_targetFramerate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vSyncCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_vSyncCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vrResolutionScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_vrResolutionScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_antiAliasingLevel(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_antiAliasingLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bloomPrePassTextureEffect(
        &mut self,
        value: crate::BeatSaber::PerformancePresets::BloomPrepassTextureEffectPreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bloomPrePassTextureEffect", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_burnMarkTrails(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_burnMarkTrails", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultAudioLatency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultAudioLatency", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_depthTexture(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_depthTexture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isDirty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDirty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mainEffectGraphics(
        &mut self,
        value: crate::BeatSaber::PerformancePresets::MainEffectPreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mainEffectGraphics", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxNumberOfCutSoundEffects(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxNumberOfCutSoundEffects", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxQueuedFrames(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxQueuedFrames", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxShockwaveParticles(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxShockwaveParticles", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_menuVRResolutionScaleMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_menuVRResolutionScaleMultiplier", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mirrorGraphics(
        &mut self,
        value: crate::BeatSaber::PerformancePresets::MirrorQualityPreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mirrorGraphics", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_obstaclesQuality(
        &mut self,
        value: crate::BeatSaber::PerformancePresets::ObstaclesQuality,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_obstaclesQuality", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_presetName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_presetName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_renderViewportScale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_renderViewportScale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_screenDisplacementEffects(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_screenDisplacementEffects", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_smokeGraphics(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_smokeGraphics", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetFramerate(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetFramerate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vSyncCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vSyncCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vrResolutionScale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vrResolutionScale", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+CustomPerformancePreset")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::PerformancePresets::CustomPerformancePreset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
