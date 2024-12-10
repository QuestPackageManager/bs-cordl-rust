#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+CompositeExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct CompositeExpression {
    __cordl_parent: crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression,
    pub _Expressions_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+CompositeExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Linq::JsonPath::CompositeExpression =>
    "Newtonsoft.Json.Linq.JsonPath"."CompositeExpression"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+CompositeExpression")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JsonPath::CompositeExpression {
    type Target = crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+CompositeExpression")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Linq::JsonPath::CompositeExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+CompositeExpression")]
impl crate::Newtonsoft::Json::Linq::JsonPath::CompositeExpression {
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
    pub fn New(
        _cordl_operator: crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_operator))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_Expressions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression,
            >,
        > = __cordl_object.invoke("get_Expressions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Expressions(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Expressions", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+CompositeExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonPath::CompositeExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
