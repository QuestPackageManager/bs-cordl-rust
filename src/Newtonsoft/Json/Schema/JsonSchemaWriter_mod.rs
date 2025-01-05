#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaWriter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
    pub _resolver: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::JsonSchemaWriter =>
    "Newtonsoft.Json.Schema"."JsonSchemaWriter"
);
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaWriter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaWriter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaWriter")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::JsonSchemaWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaWriter")]
impl crate::Newtonsoft::Json::Schema::JsonSchemaWriter {
    pub fn New(
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        resolver: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (writer, resolver))?;
        Ok(__cordl_object.into())
    }
    pub fn ReferenceOrWriteSchema(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReferenceOrWriteSchema", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteItems(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteItems", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn WritePropertyIfNotNull(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WritePropertyIfNotNull", (writer, propertyName, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteSchema(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSchema", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteSchemaDictionaryIfNotNull(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        properties: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteSchemaDictionaryIfNotNull",
                (writer, propertyName, properties),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteType(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        _cordl_type: crate::Newtonsoft::Json::Schema::JsonSchemaType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteType", (propertyName, writer, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        resolver: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (writer, resolver))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::JsonSchemaWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
