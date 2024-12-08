#[cfg(feature = "TimeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TimeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TimeExtensions => ""."TimeExtensions"
);
#[cfg(feature = "TimeExtensions")]
impl std::ops::Deref for TimeExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimeExtensions")]
impl std::ops::DerefMut for TimeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimeExtensions")]
impl TimeExtensions {}
#[cfg(feature = "TimeExtensions")]
impl quest_hook::libil2cpp::ObjectType for TimeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
