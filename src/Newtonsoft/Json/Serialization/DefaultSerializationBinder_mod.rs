#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultSerializationBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultSerializationBinder {
    __cordl_parent: crate::System::Runtime::Serialization::SerializationBinder,
    pub _typeCache: *mut crate::Newtonsoft::Json::Utilities::ThreadSafeStore_2<
        crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
        *mut crate::System::Type,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultSerializationBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::DefaultSerializationBinder =>
    "Newtonsoft.Json.Serialization"."DefaultSerializationBinder"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultSerializationBinder")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::DefaultSerializationBinder {
    type Target = crate::System::Runtime::Serialization::SerializationBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultSerializationBinder")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::DefaultSerializationBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultSerializationBinder")]
impl crate::Newtonsoft::Json::Serialization::DefaultSerializationBinder {
    pub fn BindToName(
        &mut self,
        serializedType: *mut crate::System::Type,
        assemblyName: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        typeName: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindToName", (serializedType, assemblyName, typeName))?;
        Ok(__cordl_ret)
    }
    pub fn BindToType(
        &mut self,
        assemblyName: *mut crate::System::String,
        typeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("BindToType", (assemblyName, typeName))?;
        Ok(__cordl_ret)
    }
    pub fn GetGenericTypeFromTypeName(
        &mut self,
        typeName: *mut crate::System::String,
        assembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetGenericTypeFromTypeName", (typeName, assembly))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeByName(
        &mut self,
        typeNameKey: crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetTypeByName", (typeNameKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeFromTypeNameKey(
        &mut self,
        typeNameKey: crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetTypeFromTypeNameKey", (typeNameKey))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultSerializationBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::DefaultSerializationBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
