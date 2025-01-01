#[cfg(feature = "TimelineUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TimelineUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TimelineUtils => ""
    ."TimelineUtils"
);
#[cfg(feature = "TimelineUtils")]
impl std::ops::Deref for crate::GlobalNamespace::TimelineUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimelineUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineUtils")]
impl crate::GlobalNamespace::TimelineUtils {}
#[cfg(feature = "TimelineUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TimelineUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}