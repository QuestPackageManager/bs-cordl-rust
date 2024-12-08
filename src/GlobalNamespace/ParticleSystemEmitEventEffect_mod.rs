#[cfg(feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior {
    __cordl_parent: crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior,
    pub _tickableManager: *mut crate::Zenject::TickableManager,
}
#[cfg(feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior
    => ""."ParticleSystemEmitEventEffect/BeatmapEditorParticleSystemEmitBehavior"
);
#[cfg(feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior")]
impl std::ops::Deref
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior {
    type Target = crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior")]
impl crate::GlobalNamespace::ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior {
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
    pub fn _ctor(
        &mut self,
        beatmapEvent: BasicBeatmapEventType,
        particleSystemParentTransform: *mut crate::UnityEngine::Transform,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: *mut IAudioTimeSource,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
        particleSystemEventControllerPool: *mut crate::GlobalNamespace::ParticleSystemEventController_Pool,
        tickableManager: *mut crate::Zenject::TickableManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapEvent,
                    particleSystemParentTransform,
                    particleSystemMaxSpawnedSystems,
                    audioTimeSource,
                    beatmapCallbacksController,
                    particleSystemEventControllerPool,
                    tickableManager,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapEvent: BasicBeatmapEventType,
        particleSystemParentTransform: *mut crate::UnityEngine::Transform,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: *mut IAudioTimeSource,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
        particleSystemEventControllerPool: *mut crate::GlobalNamespace::ParticleSystemEventController_Pool,
        tickableManager: *mut crate::Zenject::TickableManager,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapEvent,
                    particleSystemParentTransform,
                    particleSystemMaxSpawnedSystems,
                    audioTimeSource,
                    beatmapCallbacksController,
                    particleSystemEventControllerPool,
                    tickableManager,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+GameplayParticleSystemEmitBehavior")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemEmitEventEffect_GameplayParticleSystemEmitBehavior {
    __cordl_parent: crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior,
    pub _pauseController: *mut PauseController,
    pub _songSpeedData: *mut SongSpeedData,
}
#[cfg(feature = "ParticleSystemEmitEventEffect+GameplayParticleSystemEmitBehavior")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ParticleSystemEmitEventEffect_GameplayParticleSystemEmitBehavior
    => ""."ParticleSystemEmitEventEffect/GameplayParticleSystemEmitBehavior"
);
#[cfg(feature = "ParticleSystemEmitEventEffect+GameplayParticleSystemEmitBehavior")]
impl std::ops::Deref
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_GameplayParticleSystemEmitBehavior {
    type Target = crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+GameplayParticleSystemEmitBehavior")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_GameplayParticleSystemEmitBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+GameplayParticleSystemEmitBehavior")]
impl crate::GlobalNamespace::ParticleSystemEmitEventEffect_GameplayParticleSystemEmitBehavior {
    pub fn HandlePauseControllerDidResume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePauseControllerDidResume", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePauseControllerDidPause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePauseControllerDidPause", ())?;
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
    pub fn _ctor(
        &mut self,
        beatmapEvent: BasicBeatmapEventType,
        particleSystemParentTransform: *mut crate::UnityEngine::Transform,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: *mut IAudioTimeSource,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
        particleSystemEventControllerPool: *mut crate::GlobalNamespace::ParticleSystemEventController_Pool,
        pauseController: *mut PauseController,
        songSpeedData: *mut SongSpeedData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapEvent,
                    particleSystemParentTransform,
                    particleSystemMaxSpawnedSystems,
                    audioTimeSource,
                    beatmapCallbacksController,
                    particleSystemEventControllerPool,
                    pauseController,
                    songSpeedData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EmitParticles(
        &mut self,
        startTime: f32,
    ) -> quest_hook::libil2cpp::Result<*mut ParticleSystemEventController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ParticleSystemEventController = __cordl_object
            .invoke("EmitParticles", (startTime))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapEvent: BasicBeatmapEventType,
        particleSystemParentTransform: *mut crate::UnityEngine::Transform,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: *mut IAudioTimeSource,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
        particleSystemEventControllerPool: *mut crate::GlobalNamespace::ParticleSystemEventController_Pool,
        pauseController: *mut PauseController,
        songSpeedData: *mut SongSpeedData,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapEvent,
                    particleSystemParentTransform,
                    particleSystemMaxSpawnedSystems,
                    audioTimeSource,
                    beatmapCallbacksController,
                    particleSystemEventControllerPool,
                    pauseController,
                    songSpeedData,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+GameplayParticleSystemEmitBehavior")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_GameplayParticleSystemEmitBehavior {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior {
    __cordl_parent: crate::System::Object,
    pub _audioTimeSource: *mut IAudioTimeSource,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _particleSystemEventControllerPoolContainer: *mut MemoryPoolContainer_1<
        *mut ParticleSystemEventController,
    >,
    pub _particleSystemParentTransform: *mut crate::UnityEngine::Transform,
    pub _particleSystemMaxSpawnedSystems: i32,
    pub _beatmapDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
}
#[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior => ""
    ."ParticleSystemEmitEventEffect/ParticleSystemEmitBehavior"
);
#[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
impl std::ops::Deref
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
impl crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior {
    pub fn HandleBeatmapEvent(
        &mut self,
        basicBeatmapEventData: *mut BasicBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEvent", (basicBeatmapEventData))?;
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
    pub fn EmitParticles(
        &mut self,
        startTime: f32,
    ) -> quest_hook::libil2cpp::Result<*mut ParticleSystemEventController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ParticleSystemEventController = __cordl_object
            .invoke("EmitParticles", (startTime))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        beatmapEvent: BasicBeatmapEventType,
        particleSystemParentTransform: *mut crate::UnityEngine::Transform,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: *mut IAudioTimeSource,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
        particleSystemEventControllerPool: *mut crate::GlobalNamespace::ParticleSystemEventController_Pool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapEvent,
                    particleSystemParentTransform,
                    particleSystemMaxSpawnedSystems,
                    audioTimeSource,
                    beatmapCallbacksController,
                    particleSystemEventControllerPool,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapEvent: BasicBeatmapEventType,
        particleSystemParentTransform: *mut crate::UnityEngine::Transform,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: *mut IAudioTimeSource,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
        particleSystemEventControllerPool: *mut crate::GlobalNamespace::ParticleSystemEventController_Pool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapEvent,
                    particleSystemParentTransform,
                    particleSystemMaxSpawnedSystems,
                    audioTimeSource,
                    beatmapCallbacksController,
                    particleSystemEventControllerPool,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemEmitEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _beatmapEvent: BasicBeatmapEventType,
    pub _particleSystemParentTransform: *mut crate::UnityEngine::Transform,
    pub _particleSystemMaxSpawnedSystems: i32,
    pub _environmentContext: EnvironmentContext,
    pub _diContainer: *mut crate::Zenject::DiContainer,
    pub _particleSystemEmitBehavior: *mut crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior,
}
#[cfg(feature = "ParticleSystemEmitEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ParticleSystemEmitEventEffect => ""
    ."ParticleSystemEmitEventEffect"
);
#[cfg(feature = "ParticleSystemEmitEventEffect")]
impl std::ops::Deref for ParticleSystemEmitEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect")]
impl std::ops::DerefMut for ParticleSystemEmitEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect")]
impl ParticleSystemEmitEventEffect {
    #[cfg(feature = "ParticleSystemEmitEventEffect+GameplayParticleSystemEmitBehavior")]
    pub type GameplayParticleSystemEmitBehavior = crate::GlobalNamespace::ParticleSystemEmitEventEffect_GameplayParticleSystemEmitBehavior;
    #[cfg(
        feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior"
    )]
    pub type BeatmapEditorParticleSystemEmitBehavior = crate::GlobalNamespace::ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior;
    #[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
    pub type ParticleSystemEmitBehavior = crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect")]
impl quest_hook::libil2cpp::ObjectType for ParticleSystemEmitEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
