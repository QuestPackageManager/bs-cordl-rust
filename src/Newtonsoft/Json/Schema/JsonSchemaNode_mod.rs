#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNode")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaNode {
    __cordl_parent: crate::System::Object,
    pub _Id_k__BackingField: *mut crate::System::String,
    pub _Schemas_k__BackingField: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    >,
    pub _Properties_k__BackingField: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    >,
    pub _PatternProperties_k__BackingField: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    >,
    pub _Items_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    >,
    pub _AdditionalProperties_k__BackingField: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    pub _AdditionalItems_k__BackingField: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::JsonSchemaNode =>
    "Newtonsoft.Json.Schema"."JsonSchemaNode"
);
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNode")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaNode {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNode")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::JsonSchemaNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNode")]
impl crate::Newtonsoft::Json::Schema::JsonSchemaNode {
    #[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNode+__c")]
    pub type __c = crate::Newtonsoft::Json::Schema::JsonSchemaNode___c;
    pub fn get_PatternProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        > = __cordl_object.invoke("get_PatternProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AdditionalItems(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AdditionalItems", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Schemas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::Newtonsoft::Json::Schema::JsonSchema,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::Newtonsoft::Json::Schema::JsonSchema,
        > = __cordl_object.invoke("get_Schemas", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AdditionalProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode = __cordl_object
            .invoke("get_AdditionalProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AdditionalItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode = __cordl_object
            .invoke("get_AdditionalItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Items(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        > = __cordl_object.invoke("get_Items", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        > = __cordl_object.invoke("get_Properties", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AdditionalProperties(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AdditionalProperties", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_JsonSchema0(
        &mut self,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_JsonSchemaNode_JsonSchema1(
        &mut self,
        source: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, schema))?;
        Ok(__cordl_ret)
    }
    pub fn Combine(
        &mut self,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode = __cordl_object
            .invoke("Combine", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn New_JsonSchema0(
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (schema))?;
        Ok(__cordl_object)
    }
    pub fn New_JsonSchemaNode_JsonSchema1(
        source: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, schema))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::JsonSchemaNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
