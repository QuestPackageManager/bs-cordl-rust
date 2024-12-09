#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineClipCapsExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TimelineClipCapsExtensions => "UnityEngine.Timeline"
    ."TimelineClipCapsExtensions"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineClipCapsExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelineClipCapsExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
impl crate::UnityEngine::Timeline::TimelineClipCapsExtensions {}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipCapsExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimelineClipCapsExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
