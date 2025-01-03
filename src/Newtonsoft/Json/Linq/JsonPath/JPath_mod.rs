#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+JPath")]
#[repr(C)]
#[derive(Debug)]
pub struct JPath {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _Filters_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
        >,
    >,
    pub _currentIndex: i32,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+JPath")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JsonPath::JPath =>
    "Newtonsoft.Json.Linq.JsonPath"."JPath"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+JPath")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JsonPath::JPath {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+JPath")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JsonPath::JPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+JPath")]
impl crate::Newtonsoft::Json::Linq::JsonPath::JPath {
    pub fn CreatePathFilter(
        member: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        scan: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JsonPath::PathFilter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePathFilter", (member, scan))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnexpectedCharacterException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonException>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonException,
        > = __cordl_object.invoke("CreateUnexpectedCharacterException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EatWhitespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EatWhitespace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureLength(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureLength", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn Evaluate_JToken_JsonSelectSettings0(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        t: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonSelectSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        > = __cordl_object.invoke("Evaluate", (root, t, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn Evaluate_List_1_JToken_JsonSelectSettings1(
        filters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
            >,
        >,
        root: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        t: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonSelectSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Evaluate", (filters, root, t, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn Match(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Match", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseArrayIndexer(
        &mut self,
        indexerCloseChar: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JsonPath::PathFilter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
        > = __cordl_object.invoke("ParseArrayIndexer", (indexerCloseChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression,
        > = __cordl_object.invoke("ParseExpression", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseIndexer(
        &mut self,
        indexerOpenChar: char,
        scan: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JsonPath::PathFilter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
        > = __cordl_object.invoke("ParseIndexer", (indexerOpenChar, scan))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseMain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseMain", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseOperator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator = __cordl_object
            .invoke("ParseOperator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParsePath(
        &mut self,
        filters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
            >,
        >,
        currentPartStartIndex: i32,
        query: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParsePath", (filters, currentPartStartIndex, query))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseQuery(
        &mut self,
        indexerCloseChar: char,
        scan: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JsonPath::PathFilter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
        > = __cordl_object.invoke("ParseQuery", (indexerCloseChar, scan))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseQuotedField(
        &mut self,
        indexerCloseChar: char,
        scan: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JsonPath::PathFilter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
        > = __cordl_object.invoke("ParseQuotedField", (indexerCloseChar, scan))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSide(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ParseSide", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadQuotedString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ReadQuotedString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadRegexString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ReadRegexString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExpression(
        &mut self,
        expressionPath: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryParseExpression", (expressionPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseValue(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryParseValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Filters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
            >,
        > = __cordl_object.invoke("get_Filters", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+JPath")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonPath::JPath {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
