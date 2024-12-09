#[cfg(feature = "TimeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TimeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TimeExtensions => ""
    ."TimeExtensions"
);
#[cfg(feature = "TimeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::TimeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimeExtensions")]
impl crate::GlobalNamespace::TimeExtensions {}
#[cfg(feature = "TimeExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TimeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
