#[cfg(feature = "EnvironmentAudioEffectsPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentAudioEffectsPlayer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSource: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
}
#[cfg(feature = "EnvironmentAudioEffectsPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentAudioEffectsPlayer
    => ""."EnvironmentAudioEffectsPlayer"
);
#[cfg(feature = "EnvironmentAudioEffectsPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentAudioEffectsPlayer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentAudioEffectsPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentAudioEffectsPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentAudioEffectsPlayer")]
impl crate::GlobalNamespace::EnvironmentAudioEffectsPlayer {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PlayEffect(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        volume: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayEffect", (clip, volume))?;
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
    pub fn get_audioSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource> = __cordl_object
            .invoke("get_audioSource", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EnvironmentAudioEffectsPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentAudioEffectsPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
