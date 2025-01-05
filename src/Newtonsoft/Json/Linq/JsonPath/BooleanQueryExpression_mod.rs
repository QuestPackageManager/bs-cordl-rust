#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct BooleanQueryExpression {
    __cordl_parent: crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression,
    pub Left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Linq::JsonPath::BooleanQueryExpression =>
    "Newtonsoft.Json.Linq.JsonPath"."BooleanQueryExpression"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Linq::JsonPath::BooleanQueryExpression {
    type Target = crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Linq::JsonPath::BooleanQueryExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
impl crate::Newtonsoft::Json::Linq::JsonPath::BooleanQueryExpression {
    pub fn EqualsWithStrictMatch(
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
        queryValue: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EqualsWithStrictMatch", (value, queryValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn EqualsWithStringCoercion(
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
        queryValue: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EqualsWithStringCoercion", (value, queryValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResult(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        t: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
            >,
        > = __cordl_object.invoke("GetResult", (root, t, o))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMatch(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        t: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonSelectSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMatch", (root, t, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchTokens(
        &mut self,
        leftResult: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        rightResult: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonSelectSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchTokens", (leftResult, rightResult, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_operator: crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator,
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_operator, left, right))?;
        Ok(__cordl_object.into())
    }
    pub fn RegexEquals(
        input: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
        pattern: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonSelectSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegexEquals", (input, pattern, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_operator: crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator,
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_operator, left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonPath::BooleanQueryExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
