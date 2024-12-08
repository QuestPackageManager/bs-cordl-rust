#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayableOutputExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableOutputExtensions
    => "UnityEngine.Playables"."PlayableOutputExtensions"
);
#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
impl std::ops::Deref for crate::UnityEngine::Playables::PlayableOutputExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Playables::PlayableOutputExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
impl crate::UnityEngine::Playables::PlayableOutputExtensions {}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Playables::PlayableOutputExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
