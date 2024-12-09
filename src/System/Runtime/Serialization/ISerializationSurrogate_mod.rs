#[cfg(feature = "System+Runtime+Serialization+ISerializationSurrogate")]
#[repr(C)]
#[derive(Debug)]
pub struct ISerializationSurrogate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Serialization+ISerializationSurrogate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::ISerializationSurrogate =>
    "System.Runtime.Serialization"."ISerializationSurrogate"
);
#[cfg(feature = "System+Runtime+Serialization+ISerializationSurrogate")]
impl std::ops::Deref for crate::System::Runtime::Serialization::ISerializationSurrogate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ISerializationSurrogate")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::ISerializationSurrogate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ISerializationSurrogate")]
impl crate::System::Runtime::Serialization::ISerializationSurrogate {
    pub fn GetObjectData(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (obj, info, context))?;
        Ok(__cordl_ret)
    }
    pub fn SetObjectData(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("SetObjectData", (obj, info, context, selector))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ISerializationSurrogate")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::ISerializationSurrogate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
