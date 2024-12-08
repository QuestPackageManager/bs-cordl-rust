#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaGenerator {
    __cordl_parent: crate::System::Object,
    pub _UndefinedSchemaIdHandling_k__BackingField: crate::Newtonsoft::Json::Schema::UndefinedSchemaIdHandling,
    pub _contractResolver: *mut crate::Newtonsoft::Json::Serialization::IContractResolver,
    pub _resolver: *mut crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
    pub _stack: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaGenerator_TypeSchema,
    >,
    pub _currentSchema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::JsonSchemaGenerator =>
    "Newtonsoft.Json.Schema"."JsonSchemaGenerator"
);
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::JsonSchemaGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator")]
impl crate::Newtonsoft::Json::Schema::JsonSchemaGenerator {
    #[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator+TypeSchema")]
    pub type TypeSchema = crate::Newtonsoft::Json::Schema::JsonSchemaGenerator_TypeSchema;
    #[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator+__c__DisplayClass23_0")]
    pub type __c__DisplayClass23_0 = crate::Newtonsoft::Json::Schema::JsonSchemaGenerator___c__DisplayClass23_0;
    pub fn AddNullType(
        &mut self,
        _cordl_type: crate::Newtonsoft::Json::Schema::JsonSchemaType,
        valueRequired: crate::Newtonsoft::Json::Required,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Schema::JsonSchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Schema::JsonSchemaType = __cordl_object
            .invoke("AddNullType", (_cordl_type, valueRequired))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateISerializableContract(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateISerializableContract", (_cordl_type, contract))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateInternal(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        valueRequired: crate::Newtonsoft::Json::Required,
        required: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("GenerateInternal", (_cordl_type, valueRequired, required))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateObjectSchema(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateObjectSchema", (_cordl_type, contract))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_JsonSchemaResolver1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        resolver: *mut crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("Generate", (_cordl_type, resolver))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_JsonSchemaResolver__cordl_bool3(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        resolver: *mut crate::Newtonsoft::Json::Schema::JsonSchemaResolver,
        rootSchemaNullable: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("Generate", (_cordl_type, resolver, rootSchemaNullable))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("Generate", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn Generate__cordl_bool2(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        rootSchemaNullable: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("Generate", (_cordl_type, rootSchemaNullable))?;
        Ok(__cordl_ret)
    }
    pub fn GetDescription(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetDescription", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetJsonSchemaType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        valueRequired: crate::Newtonsoft::Json::Required,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Schema::JsonSchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Schema::JsonSchemaType = __cordl_object
            .invoke("GetJsonSchemaType", (_cordl_type, valueRequired))?;
        Ok(__cordl_ret)
    }
    pub fn GetTitle(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTitle", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeId(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        explicitOnly: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTypeId", (_cordl_type, explicitOnly))?;
        Ok(__cordl_ret)
    }
    pub fn HasFlag(
        &mut self,
        value: crate::Newtonsoft::Json::DefaultValueHandling,
        flag: crate::Newtonsoft::Json::DefaultValueHandling,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFlag", (value, flag))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Pop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaGenerator_TypeSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchemaGenerator_TypeSchema = __cordl_object
            .invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn Push(
        &mut self,
        typeSchema: *mut crate::Newtonsoft::Json::Schema::JsonSchemaGenerator_TypeSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (typeSchema))?;
        Ok(__cordl_ret)
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
    pub fn get_ContractResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::IContractResolver,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::IContractResolver = __cordl_object
            .invoke("get_ContractResolver", ())?;
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
    pub fn get_UndefinedSchemaIdHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Schema::UndefinedSchemaIdHandling,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Schema::UndefinedSchemaIdHandling = __cordl_object
            .invoke("get_UndefinedSchemaIdHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ContractResolver(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Serialization::IContractResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContractResolver", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UndefinedSchemaIdHandling(
        &mut self,
        value: crate::Newtonsoft::Json::Schema::UndefinedSchemaIdHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UndefinedSchemaIdHandling", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::JsonSchemaGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator+TypeSchema")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaGenerator_TypeSchema {
    __cordl_parent: crate::System::Object,
    pub _Type_k__BackingField: *mut crate::System::Type,
    pub _Schema_k__BackingField: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator+TypeSchema")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Schema::JsonSchemaGenerator_TypeSchema =>
    "Newtonsoft.Json.Schema"."JsonSchemaGenerator/TypeSchema"
);
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator+TypeSchema")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Schema::JsonSchemaGenerator_TypeSchema {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator+TypeSchema")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Schema::JsonSchemaGenerator_TypeSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator+TypeSchema")]
impl crate::Newtonsoft::Json::Schema::JsonSchemaGenerator_TypeSchema {
    pub fn New(
        _cordl_type: *mut crate::System::Type,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, schema))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, schema))?;
        Ok(__cordl_ret)
    }
    pub fn get_Schema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("get_Schema", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaGenerator+TypeSchema")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::JsonSchemaGenerator_TypeSchema {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}