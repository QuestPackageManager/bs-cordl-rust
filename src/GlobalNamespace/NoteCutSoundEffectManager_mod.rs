#[cfg(feature = "NoteCutSoundEffectManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutSoundEffectManager_InitData {
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useTestAudioClips, ignoreBadCuts))?;
        Ok(__cordl_object)
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
        Ok(__cordl_ret)
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
#[cfg(feature = "NoteCutSoundEffectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutSoundEffectManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioManager: *mut crate::GlobalNamespace::AudioManagerSO,
    pub _audioSamplesBeatAlignOffset: f32,
    pub _longCutEffectsAudioClips: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _shortCutEffectsAudioClips: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _testAudioClip: *mut crate::UnityEngine::AudioClip,
    pub _initData: *mut crate::GlobalNamespace::NoteCutSoundEffectManager_InitData,
    pub _beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
    pub _saberManager: *mut crate::GlobalNamespace::SaberManager,
    pub _noteCutSoundEffectPool: *mut crate::GlobalNamespace::NoteCutSoundEffect_Pool,
    pub _audioTimeSyncController: *mut crate::GlobalNamespace::AudioTimeSyncController,
    pub _handleWrongSaberTypeAsGood_k__BackingField: bool,
    pub _randomLongCutSoundPicker: *mut crate::GlobalNamespace::RandomObjectPicker_1<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _randomShortCutSoundPicker: *mut crate::GlobalNamespace::RandomObjectPicker_1<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _prevNoteATime: f32,
    pub _prevNoteBTime: f32,
    pub _prevNoteASoundEffect: *mut crate::GlobalNamespace::NoteCutSoundEffect,
    pub _prevNoteBSoundEffect: *mut crate::GlobalNamespace::NoteCutSoundEffect,
    pub _beatAlignOffset: f32,
    pub _useTestAudioClips: bool,
    pub _noteCutSoundEffectPoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::NoteCutSoundEffect,
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
        noteCutSoundEffect: *mut crate::GlobalNamespace::NoteCutSoundEffect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteCutSoundEffectDidFinish", (noteCutSoundEffect))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteDidStartDissolving(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteControllerBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteDidStartDissolving", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasSpawned(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasSpawned", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn IsSupportedNote(
        &mut self,
        noteData: *mut crate::GlobalNamespace::NoteData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSupportedNote", (noteData))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
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
    pub fn get_handleWrongSaberTypeAsGood(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_handleWrongSaberTypeAsGood", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
