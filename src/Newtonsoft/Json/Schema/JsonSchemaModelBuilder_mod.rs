#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModelBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaModelBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _nodes: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Schema::JsonSchemaNodeCollection,
    >,
    pub _nodeModels: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
            quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaModel>,
        >,
    >,
    pub _node: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModelBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Schema::JsonSchemaModelBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Schema";
    const CLASS_NAME: &'static str = "JsonSchemaModelBuilder";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        parentNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAdditionalItems", (parentNode, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddAdditionalProperties(
        &mut self,
        parentNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAdditionalProperties", (parentNode, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddItem(
        &mut self,
        parentNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
        index: i32,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItem", (parentNode, index, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddProperties(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
            >,
        >,
        target: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Schema::JsonSchemaNode,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddProperties", (source, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddProperty(
        &mut self,
        target: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Schema::JsonSchemaNode,
                >,
            >,
        >,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddProperty", (target, propertyName, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSchema(
        &mut self,
        existingNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        > = __cordl_object.invoke("AddSchema", (existingNode, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaModel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        > = __cordl_object.invoke("Build", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildNodeModel(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaModel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        > = __cordl_object.invoke("BuildNodeModel", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
