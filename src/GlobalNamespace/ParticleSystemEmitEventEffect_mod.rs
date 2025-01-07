#[cfg(feature = "ParticleSystemEmitEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemEmitEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _beatmapEvent: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _particleSystemParentTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _particleSystemMaxSpawnedSystems: i32,
    pub _environmentContext: crate::GlobalNamespace::EnvironmentContext,
    pub _diContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _particleSystemEmitBehavior: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior,
    >,
}
#[cfg(feature = "ParticleSystemEmitEventEffect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ParticleSystemEmitEventEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ParticleSystemEmitEventEffect";
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
    pub _tickableManager: quest_hook::libil2cpp::Gc<crate::Zenject::TickableManager>,
}
#[cfg(feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEditorParticleSystemEmitBehavior";
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
#[cfg(feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior")]
impl AsRef<crate::Zenject::ITickable>
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior {
    fn as_ref(&self) -> &crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+BeatmapEditorParticleSystemEmitBehavior")]
impl AsMut<crate::Zenject::ITickable>
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_BeatmapEditorParticleSystemEmitBehavior {
    fn as_mut(&mut self) -> &mut crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+GameplayParticleSystemEmitBehavior")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemEmitEventEffect_GameplayParticleSystemEmitBehavior {
    __cordl_parent: crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior,
    pub _pauseController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PauseController,
    >,
    pub _songSpeedData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SongSpeedData>,
}
#[cfg(feature = "ParticleSystemEmitEventEffect+GameplayParticleSystemEmitBehavior")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_GameplayParticleSystemEmitBehavior {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayParticleSystemEmitBehavior";
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
    pub _audioTimeSource: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAudioTimeSource,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _particleSystemEventControllerPoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ParticleSystemEventController,
            >,
        >,
    >,
    pub _particleSystemParentTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _particleSystemMaxSpawnedSystems: i32,
    pub _beatmapDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
}
#[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ParticleSystemEmitBehavior";
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
#[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ParticleSystemEmitEventEffect+ParticleSystemEmitBehavior")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::ParticleSystemEmitEventEffect_ParticleSystemEmitBehavior {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
