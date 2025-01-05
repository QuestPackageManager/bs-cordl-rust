#[cfg(feature = "System+Runtime+Serialization+SerializationEventsCache")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationEventsCache {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn GetSerializationEventsForType(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationEvents,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationEvents,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSerializationEventsForType", (t))?;
        Ok(__cordl_ret.into())
    }
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
