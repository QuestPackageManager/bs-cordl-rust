#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct DiscriminatedUnionConverter {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter,
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter =>
    "Newtonsoft.Json.Converters"."DiscriminatedUnionConverter"
);
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter {
    type Target = crate::Newtonsoft::Json::JsonConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter")]
impl crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter {
    pub const CasePropertyName: &'static str = "Case";
    pub const FieldsPropertyName: &'static str = "Fields";
    #[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+UnionCase")]
    pub type UnionCase = crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase;
    #[cfg(
        feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+__c__DisplayClass9_0"
    )]
    pub type __c__DisplayClass9_0 = crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter___c__DisplayClass9_0;
    #[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+Union")]
    pub type Union = crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union;
    #[cfg(
        feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+__c__DisplayClass8_0"
    )]
    pub type __c__DisplayClass8_0 = crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter___c__DisplayClass8_0;
    pub fn CanConvert(
        &mut self,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanConvert", (objectType))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ReadJson(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        existingValue: *mut crate::System::Object,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadJson", (reader, objectType, existingValue, serializer))?;
        Ok(__cordl_ret)
    }
    pub fn WriteJson(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteJson", (writer, value, serializer))?;
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
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+Union")]
#[repr(C)]
#[derive(Debug)]
pub struct DiscriminatedUnionConverter_Union {
    __cordl_parent: crate::System::Object,
    pub TagReader: *mut crate::Newtonsoft::Json::Utilities::FSharpFunction,
    pub Cases: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+Union")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union =>
    "Newtonsoft.Json.Converters"."DiscriminatedUnionConverter/Union"
);
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+Union")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+Union")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+Union")]
impl crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union {
    pub fn New(
        tagReader: *mut crate::Newtonsoft::Json::Utilities::FSharpFunction,
        cases: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagReader, cases))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        tagReader: *mut crate::Newtonsoft::Json::Utilities::FSharpFunction,
        cases: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagReader, cases))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+Union")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+UnionCase")]
#[repr(C)]
#[derive(Debug)]
pub struct DiscriminatedUnionConverter_UnionCase {
    __cordl_parent: crate::System::Object,
    pub Tag: i32,
    pub Name: *mut crate::System::String,
    pub Fields: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Reflection::PropertyInfo,
    >,
    pub FieldReader: *mut crate::Newtonsoft::Json::Utilities::FSharpFunction,
    pub Constructor: *mut crate::Newtonsoft::Json::Utilities::FSharpFunction,
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+UnionCase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase =>
    "Newtonsoft.Json.Converters"."DiscriminatedUnionConverter/UnionCase"
);
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+UnionCase")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+UnionCase")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+UnionCase")]
impl crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase {
    pub fn New(
        tag: i32,
        name: *mut crate::System::String,
        fields: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::PropertyInfo,
        >,
        fieldReader: *mut crate::Newtonsoft::Json::Utilities::FSharpFunction,
        constructor: *mut crate::Newtonsoft::Json::Utilities::FSharpFunction,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tag, name, fields, fieldReader, constructor))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        tag: i32,
        name: *mut crate::System::String,
        fields: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::PropertyInfo,
        >,
        fieldReader: *mut crate::Newtonsoft::Json::Utilities::FSharpFunction,
        constructor: *mut crate::Newtonsoft::Json::Utilities::FSharpFunction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tag, name, fields, fieldReader, constructor))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+UnionCase")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
