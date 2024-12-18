#[cfg(feature = "System+Runtime+Serialization+SerializationEventsCache")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationEventsCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEventsCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::SerializationEventsCache =>
    "System.Runtime.Serialization"."SerializationEventsCache"
);
#[cfg(feature = "System+Runtime+Serialization+SerializationEventsCache")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::SerializationEventsCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEventsCache")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::SerializationEventsCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEventsCache")]
impl crate::System::Runtime::Serialization::SerializationEventsCache {
    #[cfg(feature = "System+Runtime+Serialization+SerializationEventsCache+__c")]
    pub type __c = crate::System::Runtime::Serialization::SerializationEventsCache___c;
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEventsCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::SerializationEventsCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
