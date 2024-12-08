#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineClipExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimelineClipExtensions =>
    "UnityEngine.Timeline"."TimelineClipExtensions"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineClipExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelineClipExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl crate::UnityEngine::Timeline::TimelineClipExtensions {}
#[cfg(feature = "UnityEngine+Timeline+TimelineClipExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimelineClipExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
