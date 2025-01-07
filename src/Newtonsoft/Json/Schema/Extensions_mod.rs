#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::Newtonsoft::Json::Schema::Extensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Schema";
    const CLASS_NAME: &'static str = "Extensions";
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
#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
impl crate::Newtonsoft::Json::Schema::Extensions {
    pub fn IsValid_ByRefMut1(
        source: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
        errorMessages: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid", (source, schema, errorMessages))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid_JToken_JsonSchema0(
        source: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid", (source, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn Validate_JToken_JsonSchema0(
        source: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Validate", (source, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn Validate_ValidationEventHandler1(
        source: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        schema: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Schema::JsonSchema>,
        validationEventHandler: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Schema::ValidationEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Validate", (source, schema, validationEventHandler))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Schema::Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
