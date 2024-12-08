#[cfg(feature = "System+Runtime+Serialization+SerializationObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationObjectManager {
    __cordl_parent: crate::System::Object,
    pub _objectSeenTable: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub _context: crate::System::Runtime::Serialization::StreamingContext,
    pub _onSerializedHandler: *mut crate::System::Runtime::Serialization::SerializationEventHandler,
}
#[cfg(feature = "System+Runtime+Serialization+SerializationObjectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::SerializationObjectManager =>
    "System.Runtime.Serialization"."SerializationObjectManager"
);
#[cfg(feature = "System+Runtime+Serialization+SerializationObjectManager")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::SerializationObjectManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationObjectManager")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::SerializationObjectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationObjectManager")]
impl crate::System::Runtime::Serialization::SerializationObjectManager {
    pub fn AddOnSerialized(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOnSerialized", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context))?;
        Ok(__cordl_object)
    }
    pub fn RaiseOnSerializedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseOnSerializedEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn RegisterObject(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterObject", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationObjectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::SerializationObjectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
