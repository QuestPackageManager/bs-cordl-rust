#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaResolver")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSchemaResolver {
    __cordl_parent: crate::System::Object,
    pub _LoadedSchemas_k__BackingField: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaResolver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::JsonSchemaResolver =>
    "Newtonsoft.Json.Schema"."JsonSchemaResolver"
);
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaResolver")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::JsonSchemaResolver {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaResolver")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::JsonSchemaResolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaResolver")]
impl crate::Newtonsoft::Json::Schema::JsonSchemaResolver {
    #[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaResolver+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::Newtonsoft::Json::Schema::JsonSchemaResolver___c__DisplayClass5_0;
    pub fn GetSchema(
        &mut self,
        reference: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Schema::JsonSchema,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Schema::JsonSchema = __cordl_object
            .invoke("GetSchema", (reference))?;
        Ok(__cordl_ret)
    }
    pub fn set_LoadedSchemas(
        &mut self,
        value: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Schema::JsonSchema,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LoadedSchemas", (value))?;
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
    pub fn get_LoadedSchemas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Schema::JsonSchema,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Schema::JsonSchema,
        > = __cordl_object.invoke("get_LoadedSchemas", ())?;
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
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaResolver")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Schema::JsonSchemaResolver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
