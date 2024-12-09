#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel")]
#[repr(C)]
#[derive(Debug)]
pub struct VoipAudioSourceHiLevel {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub initialPlaybackDelayMS: i32,
    pub audioSource: *mut crate::UnityEngine::AudioSource,
    pub peakAmplitude: f32,
    pub pcmSource: *mut crate::Oculus::Platform::IVoipPCMSource,
}
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::VoipAudioSourceHiLevel =>
    "Oculus.Platform"."VoipAudioSourceHiLevel"
);
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel")]
impl std::ops::Deref for crate::Oculus::Platform::VoipAudioSourceHiLevel {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel")]
impl std::ops::DerefMut for crate::Oculus::Platform::VoipAudioSourceHiLevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel")]
impl crate::Oculus::Platform::VoipAudioSourceHiLevel {
    #[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel+FilterReadDelegate")]
    pub type FilterReadDelegate = crate::Oculus::Platform::VoipAudioSourceHiLevel_FilterReadDelegate;
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
    pub fn CreatePCMSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreatePCMSource", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SampleRateToEnum(
        &mut self,
        rate: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipSampleRate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Oculus::Platform::VoipSampleRate = __cordl_object
            .invoke("SampleRateToEnum", (rate))?;
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
    pub fn Stop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
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
    pub fn set_senderID(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_senderID", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::VoipAudioSourceHiLevel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel+FilterReadDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct VoipAudioSourceHiLevel_FilterReadDelegate {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub parent: *mut crate::Oculus::Platform::VoipAudioSourceHiLevel,
    pub scratchBuffer: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
}
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel+FilterReadDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::VoipAudioSourceHiLevel_FilterReadDelegate => "Oculus.Platform"
    ."VoipAudioSourceHiLevel/FilterReadDelegate"
);
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel+FilterReadDelegate")]
impl std::ops::Deref
for crate::Oculus::Platform::VoipAudioSourceHiLevel_FilterReadDelegate {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel+FilterReadDelegate")]
impl std::ops::DerefMut
for crate::Oculus::Platform::VoipAudioSourceHiLevel_FilterReadDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel+FilterReadDelegate")]
impl crate::Oculus::Platform::VoipAudioSourceHiLevel_FilterReadDelegate {
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnAudioFilterRead(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        channels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAudioFilterRead", (data, channels))?;
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
#[cfg(feature = "Oculus+Platform+VoipAudioSourceHiLevel+FilterReadDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::VoipAudioSourceHiLevel_FilterReadDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
