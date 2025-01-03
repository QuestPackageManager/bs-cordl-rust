#[cfg(feature = "Newtonsoft+Json+Serialization+SerializationBinderAdapter")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationBinderAdapter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub SerializationBinder: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::SerializationBinder,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+SerializationBinderAdapter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::SerializationBinderAdapter =>
    "Newtonsoft.Json.Serialization"."SerializationBinderAdapter"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+SerializationBinderAdapter")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::SerializationBinderAdapter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+SerializationBinderAdapter")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::SerializationBinderAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+SerializationBinderAdapter")]
impl crate::Newtonsoft::Json::Serialization::SerializationBinderAdapter {
    pub fn BindToName(
        &mut self,
        serializedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        assemblyName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        typeName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindToName", (serializedType, assemblyName, typeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindToType(
        &mut self,
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("BindToType", (assemblyName, typeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        serializationBinder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializationBinder))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        serializationBinder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializationBinder))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+SerializationBinderAdapter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::SerializationBinderAdapter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+SerializationBinderAdapter")]
impl AsRef<crate::Newtonsoft::Json::Serialization::ISerializationBinder>
for crate::Newtonsoft::Json::Serialization::SerializationBinderAdapter {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Serialization::ISerializationBinder {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+SerializationBinderAdapter")]
impl AsMut<crate::Newtonsoft::Json::Serialization::ISerializationBinder>
for crate::Newtonsoft::Json::Serialization::SerializationBinderAdapter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Newtonsoft::Json::Serialization::ISerializationBinder {
        unsafe { std::mem::transmute(self) }
    }
}
