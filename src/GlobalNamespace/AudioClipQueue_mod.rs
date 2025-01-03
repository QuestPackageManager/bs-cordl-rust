#[cfg(feature = "AudioClipQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioClipQueue {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSource: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    pub _queue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::AudioClipQueue_AudioClipWithDelay,
        >,
    >,
}
#[cfg(feature = "AudioClipQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AudioClipQueue => ""
    ."AudioClipQueue"
);
#[cfg(feature = "AudioClipQueue")]
impl std::ops::Deref for crate::GlobalNamespace::AudioClipQueue {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipQueue")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioClipQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipQueue")]
impl crate::GlobalNamespace::AudioClipQueue {
    #[cfg(feature = "AudioClipQueue+AudioClipWithDelay")]
    pub type AudioClipWithDelay = crate::GlobalNamespace::AudioClipQueue_AudioClipWithDelay;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PlayAudioClipWithDelay(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayAudioClipWithDelay", (audioClip, delay))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
}
#[cfg(feature = "AudioClipQueue")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AudioClipQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AudioClipQueue+AudioClipWithDelay")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioClipQueue_AudioClipWithDelay {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub delay: f32,
}
#[cfg(feature = "AudioClipQueue+AudioClipWithDelay")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AudioClipQueue_AudioClipWithDelay => ""
    ."AudioClipQueue/AudioClipWithDelay"
);
#[cfg(feature = "AudioClipQueue+AudioClipWithDelay")]
impl std::ops::Deref for crate::GlobalNamespace::AudioClipQueue_AudioClipWithDelay {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipQueue+AudioClipWithDelay")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioClipQueue_AudioClipWithDelay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipQueue+AudioClipWithDelay")]
impl crate::GlobalNamespace::AudioClipQueue_AudioClipWithDelay {
    pub fn New(
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (audioClip, delay))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (audioClip, delay))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioClipQueue+AudioClipWithDelay")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AudioClipQueue_AudioClipWithDelay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
