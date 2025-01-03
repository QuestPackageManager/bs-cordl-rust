#[cfg(feature = "System+Linq+Expressions+ScopeExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct ScopeExpression {
    __cordl_parent: crate::System::Linq::Expressions::BlockExpression,
    pub _variables: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+ScopeExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::ScopeExpression =>
    "System.Linq.Expressions"."ScopeExpression"
);
#[cfg(feature = "System+Linq+Expressions+ScopeExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::ScopeExpression {
    type Target = crate::System::Linq::Expressions::BlockExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ScopeExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::ScopeExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ScopeExpression")]
impl crate::System::Linq::Expressions::ScopeExpression {
    pub fn GetOrMakeVariables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        > = __cordl_object.invoke("GetOrMakeVariables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        variables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (variables))?;
        Ok(__cordl_object.into())
    }
    pub fn ReuseOrValidateVariables(
        &mut self,
        variables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        > = __cordl_object.invoke("ReuseOrValidateVariables", (variables))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        variables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (variables))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_VariablesList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        > = __cordl_object.invoke("get_VariablesList", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+ScopeExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::ScopeExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
