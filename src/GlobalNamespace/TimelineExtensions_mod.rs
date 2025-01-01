#[cfg(feature = "TimelineExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TimelineExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TimelineExtensions => ""
    ."TimelineExtensions"
);
#[cfg(feature = "TimelineExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::TimelineExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimelineExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineExtensions")]
impl crate::GlobalNamespace::TimelineExtensions {
    #[cfg(feature = "TimelineExtensions+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::GlobalNamespace::TimelineExtensions___c__DisplayClass0_0;
}
#[cfg(feature = "TimelineExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TimelineExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}