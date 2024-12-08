#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+JPath")]
#[repr(C)]
#[derive(Debug)]
pub struct JPath {
    __cordl_parent: crate::System::Object,
    pub _expression: *mut crate::System::String,
    pub _Filters_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
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
    type Target = crate::System::Object;
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
    pub fn CreateUnexpectedCharacterException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::JsonException> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::JsonException = __cordl_object
            .invoke("CreateUnexpectedCharacterException", ())?;
        Ok(__cordl_ret)
    }
    pub fn EatWhitespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EatWhitespace", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnsureLength(
        &mut self,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureLength", (message))?;
        Ok(__cordl_ret)
    }
    pub fn Evaluate(
        &mut self,
        root: *mut crate::Newtonsoft::Json::Linq::JToken,
        t: *mut crate::Newtonsoft::Json::Linq::JToken,
        settings: *mut crate::Newtonsoft::Json::Linq::JsonSelectSettings,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        > = __cordl_object.invoke("Evaluate", (root, t, settings))?;
        Ok(__cordl_ret)
    }
    pub fn Match(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Match", (s))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        expression: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression))?;
        Ok(__cordl_object)
    }
    pub fn ParseArrayIndexer(
        &mut self,
        indexerCloseChar: char,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter = __cordl_object
            .invoke("ParseArrayIndexer", (indexerCloseChar))?;
        Ok(__cordl_ret)
    }
    pub fn ParseExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression = __cordl_object
            .invoke("ParseExpression", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseIndexer(
        &mut self,
        indexerOpenChar: char,
        scan: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter = __cordl_object
            .invoke("ParseIndexer", (indexerOpenChar, scan))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseMain", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ParsePath(
        &mut self,
        filters: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
        >,
        currentPartStartIndex: i32,
        query: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParsePath", (filters, currentPartStartIndex, query))?;
        Ok(__cordl_ret)
    }
    pub fn ParseQuery(
        &mut self,
        indexerCloseChar: char,
        scan: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter = __cordl_object
            .invoke("ParseQuery", (indexerCloseChar, scan))?;
        Ok(__cordl_ret)
    }
    pub fn ParseQuotedField(
        &mut self,
        indexerCloseChar: char,
        scan: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter = __cordl_object
            .invoke("ParseQuotedField", (indexerCloseChar, scan))?;
        Ok(__cordl_ret)
    }
    pub fn ParseSide(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ParseSide", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadQuotedString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadQuotedString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadRegexString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadRegexString", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn TryParseValue(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryParseValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        expression: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression))?;
        Ok(__cordl_ret)
    }
    pub fn get_Filters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
        > = __cordl_object.invoke("get_Filters", ())?;
        Ok(__cordl_ret)
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
