#[cfg(feature = "RandomizedPitch")]
#[repr(C)]
#[derive(Debug)]
pub struct RandomizedPitch {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSource: *mut crate::UnityEngine::AudioSource,
    pub _minPitchMultiplier: f32,
    pub _maxPitchMultiplier: f32,
    pub _playOnAwake: bool,
    pub _originalPitch: f32,
    pub _restoringCoroutine: *mut crate::UnityEngine::Coroutine,
}
#[cfg(feature = "RandomizedPitch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for RandomizedPitch => ""."RandomizedPitch"
);
#[cfg(feature = "RandomizedPitch")]
impl std::ops::Deref for RandomizedPitch {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RandomizedPitch")]
impl std::ops::DerefMut for RandomizedPitch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RandomizedPitch")]
impl RandomizedPitch {
    #[cfg(feature = "RandomizedPitch+_RestorePitchWithDelay_d__10")]
    pub type _RestorePitchWithDelay_d__10 = crate::GlobalNamespace::RandomizedPitch__RestorePitchWithDelay_d__10;
    #[cfg(feature = "RandomizedPitch+_PlayDelayedCoroutine_d__9")]
    pub type _PlayDelayedCoroutine_d__9 = crate::GlobalNamespace::RandomizedPitch__PlayDelayedCoroutine_d__9;
    pub fn PlayDelayedCoroutine(
        &mut self,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("PlayDelayedCoroutine", (delay))?;
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
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn PlayDelayed(
        &mut self,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayDelayed", (delay))?;
        Ok(__cordl_ret)
    }
    pub fn Play(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Play", ())?;
        Ok(__cordl_ret)
    }
    pub fn RestorePitchWithDelay(
        &mut self,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("RestorePitchWithDelay", (delay))?;
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
#[cfg(feature = "RandomizedPitch")]
impl quest_hook::libil2cpp::ObjectType for RandomizedPitch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
