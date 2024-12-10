#[cfg(feature = "ParticleSystemEmitEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemEmitEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _beatmapEvent: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _particleSystemParentTransform: *mut crate::UnityEngine::Transform,
    pub _particleSystemMaxSpawnedSystems: i32,
    pub _environmentContext: crate::GlobalNamespace::EnvironmentContext,
    pub _diContainer: *mut crate::Zenject::DiContainer,
    pub _particleSystemEmitBehavior: *mut crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior,
}
#[cfg(feature = "ParticleSystemEmitEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ParticleSystemEmitEventEffect
    => ""."ParticleSystemEmitEventEffect"
);
#[cfg(feature = "ParticleSystemEmitEventEffect")]
impl std::ops::Deref for crate::GlobalNamespace::ParticleSystemEmitEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::ParticleSystemEmitEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect")]
impl crate::GlobalNamespace::ParticleSystemEmitEventEffect {
    #[cfg(
        feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior"
    )]
    pub type BeatmapEditorParticleSystemEmitBehavior = crate::GlobalNamespace::ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior;
    #[cfg(feature = "ParticleSystemEmitEventEffect+GameplayParticleSystemEmitBehavior")]
    pub type GameplayParticleSystemEmitBehavior = crate::GlobalNamespace::ParticleSystemEmitEventEffect_GameplayParticleSystemEmitBehavior;
    #[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
    pub type ParticleSystemEmitBehavior = crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "ParticleSystemEmitEventEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ParticleSystemEmitEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beatmapEvent: crate::GlobalNamespace::BasicBeatmapEventType,
        particleSystemParentTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAudioTimeSource,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
        particleSystemEventControllerPool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ParticleSystemEventController_Pool,
        >,
        tickableManager: quest_hook::libil2cpp::Gc<crate::Zenject::TickableManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapEvent: crate::GlobalNamespace::BasicBeatmapEventType,
        particleSystemParentTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAudioTimeSource,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
        particleSystemEventControllerPool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ParticleSystemEventController_Pool,
        >,
        tickableManager: quest_hook::libil2cpp::Gc<crate::Zenject::TickableManager>,
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
        Ok(__cordl_ret.into())
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
    pub _pauseController: *mut crate::GlobalNamespace::PauseController,
    pub _songSpeedData: *mut crate::GlobalNamespace::SongSpeedData,
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
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EmitParticles(
        &mut self,
        startTime: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ParticleSystemEventController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ParticleSystemEventController,
        > = __cordl_object.invoke("EmitParticles", (startTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePauseControllerDidPause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePauseControllerDidPause", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePauseControllerDidResume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePauseControllerDidResume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beatmapEvent: crate::GlobalNamespace::BasicBeatmapEventType,
        particleSystemParentTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAudioTimeSource,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
        particleSystemEventControllerPool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ParticleSystemEventController_Pool,
        >,
        pauseController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PauseController,
        >,
        songSpeedData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SongSpeedData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapEvent: crate::GlobalNamespace::BasicBeatmapEventType,
        particleSystemParentTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAudioTimeSource,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
        particleSystemEventControllerPool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ParticleSystemEventController_Pool,
        >,
        pauseController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PauseController,
        >,
        songSpeedData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SongSpeedData>,
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
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
    pub _beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    pub _particleSystemEventControllerPoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::ParticleSystemEventController,
    >,
    pub _particleSystemParentTransform: *mut crate::UnityEngine::Transform,
    pub _particleSystemMaxSpawnedSystems: i32,
    pub _beatmapDataCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EmitParticles(
        &mut self,
        startTime: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ParticleSystemEventController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ParticleSystemEventController,
        > = __cordl_object.invoke("EmitParticles", (startTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapEvent(
        &mut self,
        basicBeatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEvent", (basicBeatmapEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beatmapEvent: crate::GlobalNamespace::BasicBeatmapEventType,
        particleSystemParentTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAudioTimeSource,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
        particleSystemEventControllerPool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ParticleSystemEventController_Pool,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapEvent: crate::GlobalNamespace::BasicBeatmapEventType,
        particleSystemParentTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
        particleSystemMaxSpawnedSystems: i32,
        audioTimeSource: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAudioTimeSource,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
        particleSystemEventControllerPool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ParticleSystemEventController_Pool,
        >,
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
        Ok(__cordl_ret.into())
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
