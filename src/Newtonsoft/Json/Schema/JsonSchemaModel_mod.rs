#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModel")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaModel {
    __cordl_parent: crate::System::Object,
    pub _Required_k__BackingField: bool,
    pub _Type_k__BackingField: crate::Newtonsoft::Json::Schema::JsonSchemaType,
    pub _MinimumLength_k__BackingField: crate::System::Nullable_1<i32>,
    pub _MaximumLength_k__BackingField: crate::System::Nullable_1<i32>,
    pub _DivisibleBy_k__BackingField: crate::System::Nullable_1<f64>,
    pub _Minimum_k__BackingField: crate::System::Nullable_1<f64>,
    pub _Maximum_k__BackingField: crate::System::Nullable_1<f64>,
    pub _ExclusiveMinimum_k__BackingField: bool,
    pub _ExclusiveMaximum_k__BackingField: bool,
    pub _MinimumItems_k__BackingField: crate::System::Nullable_1<i32>,
    pub _MaximumItems_k__BackingField: crate::System::Nullable_1<i32>,
    pub _Patterns_k__BackingField: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::System::String,
    >,
    pub _Items_k__BackingField: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    >,
    pub _Properties_k__BackingField: *mut crate::System::Collections::Generic::IDictionary_2<
        *mut crate::System::String,
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    >,
    pub _PatternProperties_k__BackingField: *mut crate::System::Collections::Generic::IDictionary_2<
        *mut crate::System::String,
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    >,
    pub _AdditionalProperties_k__BackingField: *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    pub _AdditionalItems_k__BackingField: *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    pub _PositionalItemsValidation_k__BackingField: bool,
    pub _AllowAdditionalProperties_k__BackingField: bool,
    pub _AllowAdditionalItems_k__BackingField: bool,
    pub _UniqueItems_k__BackingField: bool,
    pub _Enum_k__BackingField: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::Newtonsoft::Json::Linq::JToken,
    >,
    pub _Disallow_k__BackingField: crate::Newtonsoft::Json::Schema::JsonSchemaType,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::JsonSchemaModel =>
    "Newtonsoft.Json.Schema"."JsonSchemaModel"
);
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModel")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModel")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::JsonSchemaModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModel")]
impl crate::Newtonsoft::Json::Schema::JsonSchemaModel {
    pub fn get_AllowAdditionalItems(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowAdditionalItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Patterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_Patterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_UniqueItems(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UniqueItems", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Required(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Required", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ExclusiveMinimum(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExclusiveMinimum", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AllowAdditionalItems(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllowAdditionalItems", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ExclusiveMinimum(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ExclusiveMinimum", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PositionalItemsValidation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_PositionalItemsValidation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaximumItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("get_MaximumItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Disallow(
        &mut self,
        value: crate::Newtonsoft::Json::Schema::JsonSchemaType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Disallow", (value))?;
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
    pub fn set_DivisibleBy(
        &mut self,
        value: crate::System::Nullable_1<f64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DivisibleBy", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Items(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        > = __cordl_object.invoke("get_Items", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AdditionalItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel = __cordl_object
            .invoke("get_AdditionalItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AdditionalItems(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AdditionalItems", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Patterns(
        &mut self,
        value: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Patterns", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Type(
        &mut self,
        value: crate::Newtonsoft::Json::Schema::JsonSchemaType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Type", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AdditionalProperties(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AdditionalProperties", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_MinimumItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("get_MinimumItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Maximum(
        &mut self,
        value: crate::System::Nullable_1<f64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Maximum", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MinimumLength(
        &mut self,
        value: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MinimumLength", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MaximumLength(
        &mut self,
        value: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaximumLength", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MinimumItems(
        &mut self,
        value: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MinimumItems", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Items(
        &mut self,
        value: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Items", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        > = __cordl_object.invoke("get_Properties", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_PositionalItemsValidation(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PositionalItemsValidation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Maximum(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f64>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<f64> = __cordl_object
            .invoke("get_Maximum", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ExclusiveMaximum(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ExclusiveMaximum", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ExclusiveMaximum(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExclusiveMaximum", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_AllowAdditionalProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_AllowAdditionalProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AllowAdditionalProperties(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllowAdditionalProperties", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Enum(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        > = __cordl_object.invoke("get_Enum", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_MaximumItems(
        &mut self,
        value: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaximumItems", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_DivisibleBy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f64>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<f64> = __cordl_object
            .invoke("get_DivisibleBy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Schema::JsonSchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Schema::JsonSchemaType = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Enum(
        &mut self,
        value: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Enum", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Disallow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Schema::JsonSchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Schema::JsonSchemaType = __cordl_object
            .invoke("get_Disallow", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Properties(
        &mut self,
        value: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Properties", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_UniqueItems(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UniqueItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Minimum(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f64>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<f64> = __cordl_object
            .invoke("get_Minimum", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MinimumLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("get_MinimumLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Required(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Required", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaximumLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("get_MaximumLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Minimum(
        &mut self,
        value: crate::System::Nullable_1<f64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Minimum", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_PatternProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        > = __cordl_object.invoke("get_PatternProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_PatternProperties(
        &mut self,
        value: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PatternProperties", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_AdditionalProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchemaModel = __cordl_object
            .invoke("get_AdditionalProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::JsonSchemaModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
