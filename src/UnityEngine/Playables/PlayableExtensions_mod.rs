#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayableExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableExtensions =>
    "UnityEngine.Playables"."PlayableExtensions"
);
#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
impl std::ops::Deref for crate::UnityEngine::Playables::PlayableExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Playables::PlayableExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
impl crate::UnityEngine::Playables::PlayableExtensions {}
#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Playables::PlayableExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
