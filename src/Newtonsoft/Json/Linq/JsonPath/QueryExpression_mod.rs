#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+QueryExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct QueryExpression {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Operator: crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+QueryExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Linq::JsonPath::QueryExpression =>
    "Newtonsoft.Json.Linq.JsonPath"."QueryExpression"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+QueryExpression")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+QueryExpression")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+QueryExpression")]
impl crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression {
    pub fn IsMatch_JToken_JToken0(
        &mut self,
        root: *mut crate::Newtonsoft::Json::Linq::JToken,
        t: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMatch", (root, t))?;
        Ok(__cordl_ret)
    }
    pub fn IsMatch_JsonSelectSettings1(
        &mut self,
        root: *mut crate::Newtonsoft::Json::Linq::JToken,
        t: *mut crate::Newtonsoft::Json::Linq::JToken,
        settings: *mut crate::Newtonsoft::Json::Linq::JsonSelectSettings,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMatch", (root, t, settings))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_operator: crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_operator))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_operator: crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_operator))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+QueryExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
