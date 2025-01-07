#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNode")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Id_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _Schemas_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
        >,
    >,
    pub _Properties_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
        >,
    >,
    pub _PatternProperties_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
        >,
    >,
    pub _Items_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
        >,
    >,
    pub _AdditionalProperties_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    >,
    pub _AdditionalItems_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Schema::JsonSchemaNode,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Schema::JsonSchemaNode {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Schema";
    const CLASS_NAME: &'static str = "JsonSchemaNode";
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
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaNode")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Combine(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        > = __cordl_object.invoke("Combine", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetId(
        schemata: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetId", (schemata))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_JsonSchema0(
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (schema))?;
        Ok(__cordl_object.into())
    }
    pub fn New_JsonSchemaNode_JsonSchema1(
        source: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, schema))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_JsonSchema0(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_JsonSchemaNode_JsonSchema1(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        >,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AdditionalItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        > = __cordl_object.invoke("get_AdditionalItems", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AdditionalProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::JsonSchemaNode,
        > = __cordl_object.invoke("get_AdditionalProperties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Items(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Schema::JsonSchemaNode,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Schema::JsonSchemaNode,
                >,
            >,
        > = __cordl_object.invoke("get_Items", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PatternProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Schema::JsonSchemaNode,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Schema::JsonSchemaNode,
                >,
            >,
        > = __cordl_object.invoke("get_PatternProperties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Schema::JsonSchemaNode,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Schema::JsonSchemaNode,
                >,
            >,
        > = __cordl_object.invoke("get_Properties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Schemas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
            >,
        > = __cordl_object.invoke("get_Schemas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AdditionalItems(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AdditionalItems", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AdditionalProperties(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchemaNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AdditionalProperties", (value))?;
        Ok(__cordl_ret.into())
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
