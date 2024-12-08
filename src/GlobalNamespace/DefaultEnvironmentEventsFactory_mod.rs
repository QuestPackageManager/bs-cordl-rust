#[cfg(feature = "DefaultEnvironmentEventsFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEnvironmentEventsFactory {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for DefaultEnvironmentEventsFactory => ""
    ."DefaultEnvironmentEventsFactory"
);
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
impl std::ops::Deref for DefaultEnvironmentEventsFactory {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
impl std::ops::DerefMut for DefaultEnvironmentEventsFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
impl DefaultEnvironmentEventsFactory {}
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
impl quest_hook::libil2cpp::ObjectType for DefaultEnvironmentEventsFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
