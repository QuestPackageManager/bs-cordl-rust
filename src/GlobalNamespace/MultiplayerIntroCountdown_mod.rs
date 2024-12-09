#[cfg(feature = "MultiplayerIntroCountdown")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerIntroCountdown {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _textAppearDuration: f32,
    pub _textDisappearDuration: f32,
    pub _goDisappearDuration: f32,
    pub _partsDistance: f32,
    pub _startLocalPosition: crate::UnityEngine::Vector3,
    pub _targetLocalPosition: crate::UnityEngine::Vector3,
    pub _readyClip: *mut crate::UnityEngine::AudioClip,
    pub _setClip: *mut crate::UnityEngine::AudioClip,
    pub _goClip: *mut crate::UnityEngine::AudioClip,
    pub _buildUpClip: *mut crate::UnityEngine::AudioClip,
    pub _textController0: *mut crate::GlobalNamespace::MultiplayerIntroCountdownTextController,
    pub _textController1: *mut crate::GlobalNamespace::MultiplayerIntroCountdownTextController,
    pub _audioSource: *mut crate::UnityEngine::AudioSource,
    pub _multiplayerOffsetByLocalPlayerPosition: *mut crate::GlobalNamespace::MultiplayerOffsetPositionByLocalPlayerPosition,
    pub _tweeningManager: *mut crate::Tweening::TimeTweeningManager,
    pub _currentTextController: *mut crate::GlobalNamespace::MultiplayerIntroCountdownTextController,
    pub _fontSize: f32,
    pub _alpha: f32,
}
#[cfg(feature = "MultiplayerIntroCountdown")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerIntroCountdown => ""
    ."MultiplayerIntroCountdown"
);
#[cfg(feature = "MultiplayerIntroCountdown")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerIntroCountdown {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerIntroCountdown")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerIntroCountdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerIntroCountdown")]
impl crate::GlobalNamespace::MultiplayerIntroCountdown {
    #[cfg(feature = "MultiplayerIntroCountdown+_CountdownRoutine_d__23")]
    pub type _CountdownRoutine_d__23 = crate::GlobalNamespace::MultiplayerIntroCountdown__CountdownRoutine_d__23;
    #[cfg(feature = "MultiplayerIntroCountdown+_PhaseRoutine_d__25")]
    pub type _PhaseRoutine_d__25 = crate::GlobalNamespace::MultiplayerIntroCountdown__PhaseRoutine_d__25;
    #[cfg(feature = "MultiplayerIntroCountdown+_PlayDelayed_d__24")]
    pub type _PlayDelayed_d__24 = crate::GlobalNamespace::MultiplayerIntroCountdown__PlayDelayed_d__24;
    #[cfg(feature = "MultiplayerIntroCountdown+__c__DisplayClass25_0")]
    pub type __c__DisplayClass25_0 = crate::GlobalNamespace::MultiplayerIntroCountdown___c__DisplayClass25_0;
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
    pub fn CountdownRoutine(
        &mut self,
        seconds: f32,
        delay: f32,
        durationMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("CountdownRoutine", (seconds, delay, durationMultiplier))?;
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
    pub fn PhaseRoutine(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
        appearDuration: f32,
        disappearDuration: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("PhaseRoutine", (text, appearDuration, disappearDuration))?;
        Ok(__cordl_ret)
    }
    pub fn PlayDelayed(
        &mut self,
        audioClip: *mut crate::UnityEngine::AudioClip,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("PlayDelayed", (audioClip, delay))?;
        Ok(__cordl_ret)
    }
    pub fn StartCountdown(
        &mut self,
        seconds: f32,
        delay: f32,
        durationMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartCountdown", (seconds, delay, durationMultiplier))?;
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
    pub fn get_textAppearDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_textAppearDuration", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerIntroCountdown")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerIntroCountdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
