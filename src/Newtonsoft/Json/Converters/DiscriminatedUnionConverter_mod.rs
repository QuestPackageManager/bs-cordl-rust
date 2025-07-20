#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct DiscriminatedUnionConverter {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter,
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Converters";
    const CLASS_NAME: &'static str = "DiscriminatedUnionConverter";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                bool,
                1usize,
            >("CanConvert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as
                    quest_hook::libil2cpp::Type > ::class(), "CanConvert", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (objectType))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnion(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union,
                >,
                1usize,
            >("CreateUnion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as
                    quest_hook::libil2cpp::Type > ::class(), "CreateUnion", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union,
        > = unsafe { method.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnionTypeLookup(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                1usize,
            >("CreateUnionTypeLookup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as
                    quest_hook::libil2cpp::Type > ::class(), "CreateUnionTypeLookup",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (t))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                4usize,
            >("ReadJson")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as
                    quest_hook::libil2cpp::Type > ::class(), "ReadJson", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method
                .invoke_unchecked(self, (reader, objectType, existingValue, serializer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteJson(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteJson")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as
                    quest_hook::libil2cpp::Type > ::class(), "WriteJson", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, value, serializer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
            quest_hook::libil2cpp::Gc<
                crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase,
            >,
        >,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+Union")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Converters";
    const CLASS_NAME: &'static str = "DiscriminatedUnionConverter/Union";
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
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase,
                >,
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
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::Utilities::FSharpFunction,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_Union as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tagReader, cases))?
        };
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
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        >,
    >,
    pub FieldReader: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Utilities::FSharpFunction,
    >,
    pub Constructor: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Utilities::FSharpFunction,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Converters+DiscriminatedUnionConverter+UnionCase")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Converters";
    const CLASS_NAME: &'static str = "DiscriminatedUnionConverter/UnionCase";
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
                quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
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
                quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
            >,
        >,
        fieldReader: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::FSharpFunction,
        >,
        constructor: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::FSharpFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::PropertyInfo,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::Utilities::FSharpFunction,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::Utilities::FSharpFunction,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::Newtonsoft::Json::Converters::DiscriminatedUnionConverter_UnionCase
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tag, name, fields, fieldReader, constructor))?
        };
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
