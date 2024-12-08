#[cfg(feature = "ParticleSystemEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lightColor0: *mut crate::GlobalNamespace::ColorSO,
    pub _lightColor1: *mut crate::GlobalNamespace::ColorSO,
    pub _highlightColor0: *mut crate::GlobalNamespace::ColorSO,
    pub _highlightColor1: *mut crate::GlobalNamespace::ColorSO,
    pub _lightOnStart: bool,
    pub _colorEvent: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _particleSystem: *mut crate::UnityEngine::ParticleSystem,
    pub _beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    pub _lightIsOn: bool,
    pub _offColor: crate::UnityEngine::Color,
    pub _highlightValue: f32,
    pub _afterHighlightColor: crate::UnityEngine::Color,
    pub _highlightColor: crate::UnityEngine::Color,
    pub kFadeSpeed: f32,
    pub _mainModule: crate::UnityEngine::ParticleSystem_MainModule,
    pub _particles: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::ParticleSystem_Particle,
    >,
    pub _particleColor: crate::UnityEngine::Color,
    pub _beatmapDataCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
}
#[cfg(feature = "ParticleSystemEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ParticleSystemEventEffect => ""
    ."ParticleSystemEventEffect"
);
#[cfg(feature = "ParticleSystemEventEffect")]
impl std::ops::Deref for crate::GlobalNamespace::ParticleSystemEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEventEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::ParticleSystemEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEventEffect")]
impl crate::GlobalNamespace::ParticleSystemEventEffect {
    pub fn HandleBeatmapEvent(
        &mut self,
        basicBeatmapEventData: *mut crate::GlobalNamespace::BasicBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEvent", (basicBeatmapEventData))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshParticles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshParticles", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(feature = "ParticleSystemEventEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ParticleSystemEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
