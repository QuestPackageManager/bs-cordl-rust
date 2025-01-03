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
    #[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+Union")]
    pub type Union = crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union;
    #[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+UnionCase")]
    pub type UnionCase = crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase;
    pub fn CanConvert(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanConvert", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnion(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CreateUnion", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnionTypeLookup(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUnionTypeLookup", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadJson(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        existingValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("ReadJson", (reader, objectType, existingValue, serializer))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteJson(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteJson", (writer, value, serializer))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub TagReader: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Utilities::FSharpFunction,
    >,
    pub Cases: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase,
        >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        tagReader: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::FSharpFunction,
        >,
        cases: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagReader, cases))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tagReader: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::FSharpFunction,
        >,
        cases: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagReader, cases))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Tag: i32,
    pub Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Fields: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Reflection::PropertyInfo>,
    >,
    pub FieldReader: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Utilities::FSharpFunction,
    >,
    pub Constructor: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Utilities::FSharpFunction,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fields: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::PropertyInfo,
            >,
        >,
        fieldReader: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::FSharpFunction,
        >,
        constructor: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::FSharpFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tag, name, fields, fieldReader, constructor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tag: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fields: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::PropertyInfo,
            >,
        >,
        fieldReader: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::FSharpFunction,
        >,
        constructor: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::FSharpFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tag, name, fields, fieldReader, constructor))?;
        Ok(__cordl_ret.into())
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
