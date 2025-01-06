#[cfg(feature = "NoteCutSoundEffectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutSoundEffectManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioManagerSO>,
    pub _audioSamplesBeatAlignOffset: f32,
    pub _longCutEffectsAudioClips: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _shortCutEffectsAudioClips: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _testAudioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutSoundEffectManager_InitData,
    >,
    pub _beatmapObjectManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectManager,
    >,
    pub _saberManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SaberManager>,
    pub _noteCutSoundEffectPool: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutSoundEffect_Pool,
    >,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _handleWrongSaberTypeAsGood_k__BackingField: bool,
    pub _randomLongCutSoundPicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RandomObjectPicker_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _randomShortCutSoundPicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RandomObjectPicker_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _prevNoteATime: f32,
    pub _prevNoteBTime: f32,
    pub _prevNoteASoundEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutSoundEffect,
    >,
    pub _prevNoteBSoundEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutSoundEffect,
    >,
    pub _beatAlignOffset: f32,
    pub _useTestAudioClips: bool,
    pub _noteCutSoundEffectPoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteCutSoundEffect>,
        >,
    >,
}
#[cfg(feature = "NoteCutSoundEffectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteCutSoundEffectManager => ""
    ."NoteCutSoundEffectManager"
);
#[cfg(feature = "NoteCutSoundEffectManager")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutSoundEffectManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutSoundEffectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager")]
impl crate::GlobalNamespace::NoteCutSoundEffectManager {
    pub const kDenseNotesVolumeMul: f32 = 0.9f32;
    pub const kMaxNumberOfEffects: i32 = 64i32;
    pub const kTwoNotesAtTheSameTimeVolumeMul: f32 = 0.9f32;
    #[cfg(feature = "NoteCutSoundEffectManager+InitData")]
    pub type InitData = crate::GlobalNamespace::NoteCutSoundEffectManager_InitData;
    pub fn HandleNoteCutSoundEffectDidFinish(
        &mut self,
        noteCutSoundEffect: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutSoundEffect,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteCutSoundEffectDidFinish", (noteCutSoundEffect))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteDidStartDissolving(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteControllerBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteDidStartDissolving", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteController,
        >,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasSpawned(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasSpawned", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSupportedNote(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSupportedNote", (noteData))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
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
    pub fn get_handleWrongSaberTypeAsGood(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_handleWrongSaberTypeAsGood", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_handleWrongSaberTypeAsGood(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_handleWrongSaberTypeAsGood", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteCutSoundEffectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutSoundEffectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteCutSoundEffectManager")]
impl AsRef<crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent>
for crate::GlobalNamespace::NoteCutSoundEffectManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager")]
impl AsMut<crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent>
for crate::GlobalNamespace::NoteCutSoundEffectManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutSoundEffectManager_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub useTestAudioClips: bool,
    pub ignoreBadCuts: bool,
}
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NoteCutSoundEffectManager_InitData => ""
    ."NoteCutSoundEffectManager/InitData"
);
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutSoundEffectManager_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutSoundEffectManager_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
impl crate::GlobalNamespace::NoteCutSoundEffectManager_InitData {
    pub fn New(
        useTestAudioClips: bool,
        ignoreBadCuts: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useTestAudioClips, ignoreBadCuts))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        useTestAudioClips: bool,
        ignoreBadCuts: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (useTestAudioClips, ignoreBadCuts))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutSoundEffectManager_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
