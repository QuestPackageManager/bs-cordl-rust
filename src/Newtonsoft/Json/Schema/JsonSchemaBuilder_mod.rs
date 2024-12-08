#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaBuilder {
    __cordl_parent: crate::System::Object,
    pub _stack: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    >,
    pub _resolver: *mut crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
    pub _documentSchemas: *mut crate::System::Collections::Generic::IDictionary_2<
        *mut crate::System::String,
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    >,
    pub _currentSchema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    pub _rootSchema: *mut crate::Newtonsoft::Json::Linq::JObject,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::JsonSchemaBuilder =>
    "Newtonsoft.Json.Schema"."JsonSchemaBuilder"
);
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaBuilder")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaBuilder")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::JsonSchemaBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaBuilder")]
impl crate::Newtonsoft::Json::Schema::JsonSchemaBuilder {
    #[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaBuilder+__c__DisplayClass23_0")]
    pub type __c__DisplayClass23_0 = crate::Newtonsoft::Json::Schema::JsonSchemaBuilder___c__DisplayClass23_0;
    pub fn ProcessSchemaProperties(
        &mut self,
        schemaObject: *mut crate::Newtonsoft::Json::Linq::JObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessSchemaProperties", (schemaObject))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessItems(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessItems", (token))?;
        Ok(__cordl_ret)
    }
    pub fn BuildSchema(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("BuildSchema", (token))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveReferences(
        &mut self,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("ResolveReferences", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessExtends(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessExtends", (token))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAdditionalItems(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAdditionalItems", (token))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAdditionalProperties(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAdditionalProperties", (token))?;
        Ok(__cordl_ret)
    }
    pub fn Pop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessProperties(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchema,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchema,
        > = __cordl_object.invoke("ProcessProperties", (token))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        resolver: *mut crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resolver))?;
        Ok(__cordl_ret)
    }
    pub fn Push(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessType(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Newtonsoft::Json::Schema::JsonSchemaType>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::Newtonsoft::Json::Schema::JsonSchemaType,
        > = __cordl_object.invoke("ProcessType", (token))?;
        Ok(__cordl_ret)
    }
    pub fn UnescapeReference(
        &mut self,
        reference: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("UnescapeReference", (reference))?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("get_CurrentSchema", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessEnum(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessEnum", (token))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("Read", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        resolver: *mut crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resolver))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::JsonSchemaBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
