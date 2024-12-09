#[cfg(feature = "BufferedAudioStream")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferedAudioStream {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub audio: *mut crate::UnityEngine::AudioSource,
    pub audioBuffer: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub writePos: i32,
    pub playbackDelayRemaining: f32,
    pub remainingBufferTime: f32,
}
#[cfg(feature = "BufferedAudioStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BufferedAudioStream => ""
    ."BufferedAudioStream"
);
#[cfg(feature = "BufferedAudioStream")]
impl std::ops::Deref for crate::GlobalNamespace::BufferedAudioStream {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BufferedAudioStream")]
impl std::ops::DerefMut for crate::GlobalNamespace::BufferedAudioStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BufferedAudioStream")]
impl crate::GlobalNamespace::BufferedAudioStream {
    pub const VerboseLogging: bool = false;
    pub const bufferLengthSeconds: f32 = 0.25f32;
    pub const bufferSize: i32 = 12000i32;
    pub const playbackDelayTimeSeconds: f32 = 0.05f32;
    pub const sampleRate: i32 = 48000i32;
    pub fn AddData(
        &mut self,
        samples: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddData", (samples))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        audio: *mut crate::UnityEngine::AudioSource,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (audio))?;
        Ok(__cordl_object)
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
        audio: *mut crate::UnityEngine::AudioSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (audio))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BufferedAudioStream")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BufferedAudioStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
