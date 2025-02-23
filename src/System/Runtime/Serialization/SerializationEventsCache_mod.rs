#[cfg(feature = "System+Runtime+Serialization+SerializationEventsCache")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationEventsCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEventsCache")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::SerializationEventsCache {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "SerializationEventsCache";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn GetSerializationEventsForType(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationEvents,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::SerializationEvents,
                >,
                1usize,
            >("GetSerializationEventsForType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSerializationEventsForType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationEvents,
        > = unsafe { method.invoke_unchecked((), (t)) };
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
