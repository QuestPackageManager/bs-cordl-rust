#[cfg(feature = "System+Linq+Expressions+RuntimeVariablesExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeVariablesExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _Variables_k__BackingField: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
        *mut crate::System::Linq::Expressions::ParameterExpression,
    >,
}
#[cfg(feature = "System+Linq+Expressions+RuntimeVariablesExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::RuntimeVariablesExpression => "System.Linq.Expressions"
    ."RuntimeVariablesExpression"
);
#[cfg(feature = "System+Linq+Expressions+RuntimeVariablesExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::RuntimeVariablesExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+RuntimeVariablesExpression")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::RuntimeVariablesExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+RuntimeVariablesExpression")]
impl crate::System::Linq::Expressions::RuntimeVariablesExpression {
    pub fn get_Variables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        > = __cordl_object.invoke("get_Variables", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+RuntimeVariablesExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::RuntimeVariablesExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
