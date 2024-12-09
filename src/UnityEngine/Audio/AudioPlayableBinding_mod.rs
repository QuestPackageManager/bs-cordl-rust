#[cfg(feature = "UnityEngine+Audio+AudioPlayableBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioPlayableBinding {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableBinding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Audio::AudioPlayableBinding =>
    "UnityEngine.Audio"."AudioPlayableBinding"
);
#[cfg(feature = "UnityEngine+Audio+AudioPlayableBinding")]
impl std::ops::Deref for crate::UnityEngine::Audio::AudioPlayableBinding {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableBinding")]
impl std::ops::DerefMut for crate::UnityEngine::Audio::AudioPlayableBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableBinding")]
impl crate::UnityEngine::Audio::AudioPlayableBinding {}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableBinding")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Audio::AudioPlayableBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
