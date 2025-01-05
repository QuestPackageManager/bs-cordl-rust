#[cfg(feature = "BeatEffectSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatEffectSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _effectDuration: f32,
    pub _bombColorEffect: crate::UnityEngine::Color,
    pub _colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
    pub _beatmapObjectManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectManager,
    >,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatEffectSpawner_InitData,
    >,
    pub _bloomFog: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomFogSO>,
    pub _songController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongController,
    >,
    pub _beatEffectPoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatEffect>,
        >,
    >,
}
#[cfg(feature = "BeatEffectSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatEffectSpawner => ""
    ."BeatEffectSpawner"
);
#[cfg(feature = "BeatEffectSpawner")]
impl std::ops::Deref for crate::GlobalNamespace::BeatEffectSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffectSpawner")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatEffectSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffectSpawner")]
impl crate::GlobalNamespace::BeatEffectSpawner {
    #[cfg(feature = "BeatEffectSpawner+InitData")]
    pub type InitData = crate::GlobalNamespace::BeatEffectSpawner_InitData;
    pub fn HandleBeatEffectDidFinish(
        &mut self,
        beatEffect: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatEffect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatEffectDidFinish", (beatEffect))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteDidStartJump(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteDidStartJump", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        beatEffectPool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatEffect_Pool,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (beatEffectPool))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(feature = "BeatEffectSpawner")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatEffectSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatEffectSpawner")]
impl AsRef<crate::GlobalNamespace::IBeatEffectDidFinishEvent>
for crate::GlobalNamespace::BeatEffectSpawner {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatEffectDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatEffectSpawner")]
impl AsMut<crate::GlobalNamespace::IBeatEffectDidFinishEvent>
for crate::GlobalNamespace::BeatEffectSpawner {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBeatEffectDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatEffectSpawner+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatEffectSpawner_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub hideNoteSpawnEffect: bool,
}
#[cfg(feature = "BeatEffectSpawner+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatEffectSpawner_InitData =>
    ""."BeatEffectSpawner/InitData"
);
#[cfg(feature = "BeatEffectSpawner+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatEffectSpawner_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffectSpawner+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatEffectSpawner_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffectSpawner+InitData")]
impl crate::GlobalNamespace::BeatEffectSpawner_InitData {
    pub fn New(
        hideNoteSpawnEffect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hideNoteSpawnEffect))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        hideNoteSpawnEffect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hideNoteSpawnEffect))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatEffectSpawner+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatEffectSpawner_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
