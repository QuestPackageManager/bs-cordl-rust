#[cfg(feature = "EnvironmentStartEndSongAudioEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentStartEndSongAudioEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _playStartSongForNonZeroStartSongTime: bool,
    pub _songFinishedAheadTime: f32,
    pub _songStartAudioClipVolume: f32,
    pub _songFinishedAudioClipVolume: f32,
    pub _songFailedAudioClipVolume: f32,
    pub _songStartAudioClips: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _songFinishedAudioClips: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _songFailedAudioClips: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _levelEndActions: *mut ILevelEndActions,
    pub _audioTimeSource: *mut IAudioTimeSource,
    pub _gamePause: *mut IGamePause,
    pub _audioEffectsPlayer: *mut EnvironmentAudioEffectsPlayer,
    pub _environmentContext: EnvironmentContext,
    pub _songFinishedPlayed: bool,
    pub _isWaitingToPlayStartAudio: bool,
}
#[cfg(feature = "EnvironmentStartEndSongAudioEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EnvironmentStartEndSongAudioEffect => ""
    ."EnvironmentStartEndSongAudioEffect"
);
#[cfg(feature = "EnvironmentStartEndSongAudioEffect")]
impl std::ops::Deref for EnvironmentStartEndSongAudioEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentStartEndSongAudioEffect")]
impl std::ops::DerefMut for EnvironmentStartEndSongAudioEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentStartEndSongAudioEffect")]
impl EnvironmentStartEndSongAudioEffect {
    pub const kSmallSongTime: f32 = 0.5f32;
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
    pub fn LevelWillFinishWithinAheadTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LevelWillFinishWithinAheadTime", ())?;
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
    pub fn HandleGamePauseDidResume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGamePauseDidResume", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelFailed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelFailed", ())?;
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
    pub fn HandleGamePauseDidPause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGamePauseDidPause", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "EnvironmentStartEndSongAudioEffect")]
impl quest_hook::libil2cpp::ObjectType for EnvironmentStartEndSongAudioEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
