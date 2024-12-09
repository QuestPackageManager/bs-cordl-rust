#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModelBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaModelBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _nodes: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNodeCollection,
    pub _nodeModels: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    >,
    pub _node: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModelBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::JsonSchemaModelBuilder
    => "Newtonsoft.Json.Schema"."JsonSchemaModelBuilder"
);
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModelBuilder")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaModelBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModelBuilder")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::JsonSchemaModelBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModelBuilder")]
impl crate::Newtonsoft::Json::Schema::JsonSchemaModelBuilder {
    pub fn AddAdditionalItems(
        &mut self,
        parentNode: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAdditionalItems", (parentNode, schema))?;
        Ok(__cordl_ret)
    }
    pub fn AddAdditionalProperties(
        &mut self,
        parentNode: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAdditionalProperties", (parentNode, schema))?;
        Ok(__cordl_ret)
    }
    pub fn AddItem(
        &mut self,
        parentNode: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        index: i32,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItem", (parentNode, index, schema))?;
        Ok(__cordl_ret)
    }
    pub fn AddProperties(
        &mut self,
        source: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::Newtonsoft::Json::Schema::JsonSchema,
        >,
        target: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddProperties", (source, target))?;
        Ok(__cordl_ret)
    }
    pub fn AddProperty(
        &mut self,
        target: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
        propertyName: *mut quest_hook::libil2cpp::Il2CppString,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddProperty", (target, propertyName, schema))?;
        Ok(__cordl_ret)
    }
    pub fn AddSchema(
        &mut self,
        existingNode: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode = __cordl_object
            .invoke("AddSchema", (existingNode, schema))?;
        Ok(__cordl_ret)
    }
    pub fn Build(
        &mut self,
        schema: *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel = __cordl_object
            .invoke("Build", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn BuildNodeModel(
        &mut self,
        node: *mut crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel = __cordl_object
            .invoke("BuildNodeModel", (node))?;
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
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModelBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::JsonSchemaModelBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
