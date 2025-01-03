#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionVisitorUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::Utils::ExpressionVisitorUtils
    => "System.Dynamic.Utils"."ExpressionVisitorUtils"
);
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
impl std::ops::Deref for crate::System::Dynamic::Utils::ExpressionVisitorUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
impl std::ops::DerefMut for crate::System::Dynamic::Utils::ExpressionVisitorUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
impl crate::System::Dynamic::Utils::ExpressionVisitorUtils {
    pub fn VisitArguments(
        visitor: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ExpressionVisitor,
        >,
        nodes: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::IArgumentProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VisitArguments", (visitor, nodes))?;
        Ok(__cordl_ret.into())
    }
    pub fn VisitBlockExpressions(
        visitor: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ExpressionVisitor,
        >,
        block: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VisitBlockExpressions", (visitor, block))?;
        Ok(__cordl_ret.into())
    }
    pub fn VisitParameters(
        visitor: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ExpressionVisitor,
        >,
        nodes: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::IParameterProvider,
        >,
        callerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VisitParameters", (visitor, nodes, callerName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::Utils::ExpressionVisitorUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
