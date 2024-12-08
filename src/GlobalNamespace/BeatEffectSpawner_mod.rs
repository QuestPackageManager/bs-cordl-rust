#[cfg(feature = "BeatEffectSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatEffectSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _effectDuration: f32,
    pub _bombColorEffect: crate::UnityEngine::Color,
    pub _colorManager: *mut crate::GlobalNamespace::ColorManager,
    pub _beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
    pub _audioTimeSyncController: *mut crate::GlobalNamespace::AudioTimeSyncController,
    pub _initData: *mut crate::GlobalNamespace::BeatEffectSpawner_InitData,
    pub _bloomFog: *mut crate::GlobalNamespace::BloomFogSO,
    pub _songController: *mut crate::GlobalNamespace::SongController,
    pub _beatEffectPoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::BeatEffect,
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
        beatEffect: *mut crate::GlobalNamespace::BeatEffect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatEffectDidFinish", (beatEffect))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteDidStartJump(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteDidStartJump", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        beatEffectPool: *mut crate::GlobalNamespace::BeatEffect_Pool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (beatEffectPool))?;
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
#[cfg(feature = "BeatEffectSpawner")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatEffectSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatEffectSpawner+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatEffectSpawner_InitData {
    __cordl_parent: crate::System::Object,
    pub hideNoteSpawnEffect: bool,
}
#[cfg(feature = "BeatEffectSpawner+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatEffectSpawner_InitData =>
    ""."BeatEffectSpawner/InitData"
);
#[cfg(feature = "BeatEffectSpawner+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatEffectSpawner_InitData {
    type Target = crate::System::Object;
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
    pub fn New(hideNoteSpawnEffect: bool) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hideNoteSpawnEffect))?;
        Ok(__cordl_object)
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
        Ok(__cordl_ret)
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
