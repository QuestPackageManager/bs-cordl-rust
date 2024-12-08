#[cfg(feature = "System+Runtime+Serialization+SerializationEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationEvents {
    __cordl_parent: crate::System::Object,
    pub _onSerializingMethods: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Reflection::MethodInfo,
    >,
    pub _onSerializedMethods: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Reflection::MethodInfo,
    >,
    pub _onDeserializingMethods: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Reflection::MethodInfo,
    >,
    pub _onDeserializedMethods: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Reflection::MethodInfo,
    >,
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEvents")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::SerializationEvents =>
    "System.Runtime.Serialization"."SerializationEvents"
);
#[cfg(feature = "System+Runtime+Serialization+SerializationEvents")]
impl std::ops::Deref for crate::System::Runtime::Serialization::SerializationEvents {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEvents")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::SerializationEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEvents")]
impl crate::System::Runtime::Serialization::SerializationEvents {
    pub fn InvokeOnSerializing(
        &mut self,
        obj: *mut crate::System::Object,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeOnSerializing", (obj, context))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeOnDeserialized(
        &mut self,
        obj: *mut crate::System::Object,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeOnDeserialized", (obj, context))?;
        Ok(__cordl_ret)
    }
    pub fn GetMethodsWithAttribute(
        &mut self,
        attribute: *mut crate::System::Type,
        t: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Reflection::MethodInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("GetMethodsWithAttribute", (attribute, t))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasOnSerializingEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasOnSerializingEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn InvokeOnDeserializing(
        &mut self,
        obj: *mut crate::System::Object,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeOnDeserializing", (obj, context))?;
        Ok(__cordl_ret)
    }
    pub fn AddOnSerialized(
        &mut self,
        obj: *mut crate::System::Object,
        handler: *mut crate::System::Runtime::Serialization::SerializationEventHandler,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::SerializationEventHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::SerializationEventHandler = __cordl_object
            .invoke("AddOnSerialized", (obj, handler))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        t: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (t))?;
        Ok(__cordl_ret)
    }
    pub fn AddOnDeserialized(
        &mut self,
        obj: *mut crate::System::Object,
        handler: *mut crate::System::Runtime::Serialization::SerializationEventHandler,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::SerializationEventHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::SerializationEventHandler = __cordl_object
            .invoke("AddOnDeserialized", (obj, handler))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        t: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (t))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEvents")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::SerializationEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
