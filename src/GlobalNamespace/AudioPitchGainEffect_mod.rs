#[cfg(feature = "AudioPitchGainEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioPitchGainEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSource: *mut crate::UnityEngine::AudioSource,
    pub _duration: f32,
    pub _pitchCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _gainCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _currentCoroutine: *mut crate::UnityEngine::Coroutine,
    pub _startPitch: f32,
    pub _startVolume: f32,
}
#[cfg(feature = "AudioPitchGainEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AudioPitchGainEffect => ""."AudioPitchGainEffect"
);
#[cfg(feature = "AudioPitchGainEffect")]
impl std::ops::Deref for AudioPitchGainEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioPitchGainEffect")]
impl std::ops::DerefMut for AudioPitchGainEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioPitchGainEffect")]
impl AudioPitchGainEffect {
    #[cfg(feature = "AudioPitchGainEffect+_StartEffectCoroutine_d__8")]
    pub type _StartEffectCoroutine_d__8 = crate::GlobalNamespace::AudioPitchGainEffect__StartEffectCoroutine_d__8;
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
    pub fn StartEffectCoroutine(
        &mut self,
        volumeScale: f32,
        finishCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("StartEffectCoroutine", (volumeScale, finishCallback))?;
        Ok(__cordl_ret)
    }
    pub fn InterruptEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InterruptEffect", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetAudioSource(
        &mut self,
        audioSource: *mut crate::UnityEngine::AudioSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAudioSource", (audioSource))?;
        Ok(__cordl_ret)
    }
    pub fn StartEffect(
        &mut self,
        volumeScale: f32,
        finishCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartEffect", (volumeScale, finishCallback))?;
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
#[cfg(feature = "AudioPitchGainEffect")]
impl quest_hook::libil2cpp::ObjectType for AudioPitchGainEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
