#[cfg(feature = "NoteCutSoundEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutSoundEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSource: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    pub _speedToVolumeCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _badCutSoundEffectAudioClips: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _badCutVolume: f32,
    pub _goodCutVolume: f32,
    pub _saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
    pub _noteController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteController,
    >,
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
    pub _badCutRandomSoundPicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RandomObjectPicker_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        >,
    >,
    pub _handleWrongSaberTypeAsGood: bool,
    pub _ignoreSaberSpeed: bool,
    pub _ignoreBadCuts: bool,
    pub _didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LazyCopyHashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent,
            >,
        >,
    >,
}
#[cfg(feature = "NoteCutSoundEffect")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::NoteCutSoundEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutSoundEffect";
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
#[cfg(feature = "NoteCutSoundEffect")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutSoundEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutSoundEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutSoundEffect")]
impl crate::GlobalNamespace::NoteCutSoundEffect {
    pub const kEndFadeLength: f32 = 0.01f32;
    pub const kEndOverlap: f32 = 100.01f32;
    #[cfg(feature = "NoteCutSoundEffect+Pool")]
    pub type Pool = crate::GlobalNamespace::NoteCutSoundEffect_Pool;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteController,
        >,
        noteDSPTime: f64,
        aheadTime: f32,
        missedTimeOffset: f32,
        timeToPrevNote: f32,
        timeToNextNote: f32,
        saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NoteDidStartDissolving(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteControllerBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteDidStartDissolving", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn NoteWasCut(
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
            .invoke("NoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnLateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLateUpdate", ())?;
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
    pub fn StopPlayingAndFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopPlayingAndFinish", ())?;
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
    pub fn get_didFinishEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::INoteCutSoundEffectDidFinishEvent,
                >,
            >,
        > = __cordl_object.invoke("get_didFinishEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_volumeMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_volumeMultiplier", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteCutSoundEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteCutSoundEffect {
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
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteCutSoundEffect>,
    >,
}
#[cfg(feature = "NoteCutSoundEffect+Pool")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteCutSoundEffect_Pool {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Pool";
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
#[cfg(feature = "NoteCutSoundEffect+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutSoundEffect_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteCutSoundEffect>,
    >;
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
