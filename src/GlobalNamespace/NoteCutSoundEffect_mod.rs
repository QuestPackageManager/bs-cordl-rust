#[cfg(feature = "NoteCutSoundEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutSoundEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSource: *mut crate::UnityEngine::AudioSource,
    pub _speedToVolumeCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _badCutSoundEffectAudioClips: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _badCutVolume: f32,
    pub _goodCutVolume: f32,
    pub _saber: *mut Saber,
    pub _noteController: *mut NoteController,
    pub _isPlaying: bool,
    pub _volumeMultiplier: f32,
    pub _noteWasCut: bool,
    pub _noteStartedDissolving: bool,
    pub _aheadTime: f32,
    pub _timeToNextNote: f32,
    pub _timeToPrevNote: f32,
    pub _startDSPTime: f64,
    pub _endDSPtime: f64,
    pub _fadeOutStartDSPtime: f64,
    pub _noteMissedTimeOffset: f32,
    pub _beforeCutVolume: f32,
    pub _goodCut: bool,
    pub _pitch: f32,
    pub _badCutRandomSoundPicker: *mut RandomObjectPicker_1<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _handleWrongSaberTypeAsGood: bool,
    pub _ignoreSaberSpeed: bool,
    pub _ignoreBadCuts: bool,
    pub _didFinishEvent: *mut LazyCopyHashSet_1<*mut INoteCutSoundEffectDidFinishEvent>,
}
#[cfg(feature = "NoteCutSoundEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NoteCutSoundEffect => ""."NoteCutSoundEffect"
);
#[cfg(feature = "NoteCutSoundEffect")]
impl std::ops::Deref for NoteCutSoundEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffect")]
impl std::ops::DerefMut for NoteCutSoundEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffect")]
impl NoteCutSoundEffect {
    pub const kEndFadeLength: f32 = 0.01f32;
    pub const kEndOverlap: f32 = 100.01f32;
    #[cfg(feature = "NoteCutSoundEffect+Pool")]
    pub type Pool = crate::GlobalNamespace::NoteCutSoundEffect_Pool;
    pub fn get_volumeMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_volumeMultiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_didFinishEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut ILazyCopyHashSet_1<*mut INoteCutSoundEffectDidFinishEvent>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ILazyCopyHashSet_1<
            *mut INoteCutSoundEffectDidFinishEvent,
        > = __cordl_object.invoke("get_didFinishEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_volumeMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_volumeMultiplier", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OnLateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        audioClip: *mut crate::UnityEngine::AudioClip,
        noteController: *mut NoteController,
        noteDSPTime: f64,
        aheadTime: f32,
        missedTimeOffset: f32,
        timeToPrevNote: f32,
        timeToNextNote: f32,
        saber: *mut Saber,
        handleWrongSaberTypeAsGood: bool,
        volumeMultiplier: f32,
        ignoreSaberSpeed: bool,
        ignoreBadCuts: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    audioClip,
                    noteController,
                    noteDSPTime,
                    aheadTime,
                    missedTimeOffset,
                    timeToPrevNote,
                    timeToNextNote,
                    saber,
                    handleWrongSaberTypeAsGood,
                    volumeMultiplier,
                    ignoreSaberSpeed,
                    ignoreBadCuts,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn NoteWasCut(
        &mut self,
        noteController: *mut NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret)
    }
    pub fn NoteDidStartDissolving(
        &mut self,
        noteController: *mut NoteControllerBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteDidStartDissolving", (noteController))?;
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
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopPlayingAndFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopPlayingAndFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn ComputeDSPTimes(
        &mut self,
        noteDSPTime: f64,
        aheadTime: f32,
        timeToPrevNote: f32,
        timeToNextNote: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ComputeDSPTimes",
                (noteDSPTime, aheadTime, timeToPrevNote, timeToNextNote),
            )?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "NoteCutSoundEffect")]
impl quest_hook::libil2cpp::ObjectType for NoteCutSoundEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteCutSoundEffect+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutSoundEffect_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<*mut NoteCutSoundEffect>,
}
#[cfg(feature = "NoteCutSoundEffect+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteCutSoundEffect_Pool => ""
    ."NoteCutSoundEffect/Pool"
);
#[cfg(feature = "NoteCutSoundEffect+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutSoundEffect_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<*mut NoteCutSoundEffect>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffect+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutSoundEffect_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffect+Pool")]
impl crate::GlobalNamespace::NoteCutSoundEffect_Pool {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "NoteCutSoundEffect+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutSoundEffect_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
