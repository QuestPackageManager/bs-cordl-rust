#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
#[repr(C)]
#[derive(Debug)]
pub struct SurrogateForCyclicalReference {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub innerSurrogate: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ISerializationSurrogate,
    >,
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::SurrogateForCyclicalReference =>
    "System.Runtime.Serialization"."SurrogateForCyclicalReference"
);
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::SurrogateForCyclicalReference {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::SurrogateForCyclicalReference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl crate::System::Runtime::Serialization::SurrogateForCyclicalReference {
    pub fn GetObjectData(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (obj, info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectData(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        selector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("SetObjectData", (obj, info, context, selector))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::SurrogateForCyclicalReference {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl AsRef<crate::System::Runtime::Serialization::ISerializationSurrogate>
for crate::System::Runtime::Serialization::SurrogateForCyclicalReference {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializationSurrogate {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl AsMut<crate::System::Runtime::Serialization::ISerializationSurrogate>
for crate::System::Runtime::Serialization::SurrogateForCyclicalReference {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::ISerializationSurrogate {
        unsafe { std::mem::transmute(self) }
    }
}
