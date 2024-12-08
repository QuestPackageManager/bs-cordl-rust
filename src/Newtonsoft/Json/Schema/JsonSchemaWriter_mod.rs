#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaWriter {
    __cordl_parent: crate::System::Object,
    pub _writer: *mut crate::Newtonsoft::Json::JsonWriter,
    pub _resolver: *mut crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::JsonSchemaWriter =>
    "Newtonsoft.Json.Schema"."JsonSchemaWriter"
);
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaWriter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaWriter {
    type Target = crate::System::Object;
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
    #[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaWriter+__c")]
    pub type __c = crate::Newtonsoft::Json::Schema::JsonSchemaWriter___c;
    pub fn New(
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        resolver: *mut crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (writer, resolver))?;
        Ok(__cordl_object)
    }
    pub fn ReferenceOrWriteSchema(
        &mut self,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReferenceOrWriteSchema", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn WriteItems(
        &mut self,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteItems", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn WritePropertyIfNotNull(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        propertyName: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WritePropertyIfNotNull", (writer, propertyName, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteSchema(
        &mut self,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSchema", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn WriteSchemaDictionaryIfNotNull(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        propertyName: *mut crate::System::String,
        properties: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchema,
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
        Ok(__cordl_ret)
    }
    pub fn WriteType(
        &mut self,
        propertyName: *mut crate::System::String,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        _cordl_type: crate::Newtonsoft::Json::Schema::JsonSchemaType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteType", (propertyName, writer, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        resolver: *mut crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (writer, resolver))?;
        Ok(__cordl_ret)
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
