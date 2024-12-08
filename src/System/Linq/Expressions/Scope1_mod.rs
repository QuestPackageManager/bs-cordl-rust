#[cfg(feature = "System+Linq+Expressions+Scope1")]
#[repr(C)]
#[derive(Debug)]
pub struct Scope1 {
    __cordl_parent: crate::System::Linq::Expressions::ScopeExpression,
    pub _body: *mut crate::System::Object,
}
#[cfg(feature = "System+Linq+Expressions+Scope1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::Scope1 =>
    "System.Linq.Expressions"."Scope1"
);
#[cfg(feature = "System+Linq+Expressions+Scope1")]
impl std::ops::Deref for crate::System::Linq::Expressions::Scope1 {
    type Target = crate::System::Linq::Expressions::ScopeExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Scope1")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Scope1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Scope1")]
impl crate::System::Linq::Expressions::Scope1 {
    pub fn GetExpression(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("GetExpression", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetOrMakeExpressions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("GetOrMakeExpressions", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Expression0(
        variables: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
        body: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (variables, body))?;
        Ok(__cordl_object)
    }
    pub fn New_Object1(
        variables: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
        body: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (variables, body))?;
        Ok(__cordl_object)
    }
    pub fn Rewrite(
        &mut self,
        variables: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::BlockExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::BlockExpression = __cordl_object
            .invoke("Rewrite", (variables, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Expression0(
        &mut self,
        variables: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
        body: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (variables, body))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object1(
        &mut self,
        variables: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
        body: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (variables, body))?;
        Ok(__cordl_ret)
    }
    pub fn get_ExpressionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ExpressionCount", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Scope1")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Expressions::Scope1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}