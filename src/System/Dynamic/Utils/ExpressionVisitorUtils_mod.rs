#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionVisitorUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Dynamic::Utils::ExpressionVisitorUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Dynamic.Utils";
    const CLASS_NAME: &'static str = "ExpressionVisitorUtils";
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
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::ExpressionVisitor,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::IArgumentProvider,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                    >,
                >,
                2usize,
            >("VisitArguments")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitArguments", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        > = unsafe { method.invoke_unchecked((), (visitor, nodes)) };
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
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::ExpressionVisitor,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::BlockExpression,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                    >,
                >,
                2usize,
            >("VisitBlockExpressions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitBlockExpressions", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        > = unsafe { method.invoke_unchecked((), (visitor, block)) };
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
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::ExpressionVisitor,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::IParameterProvider,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::ParameterExpression,
                        >,
                    >,
                >,
                3usize,
            >("VisitParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitParameters", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (visitor, nodes, callerName)) };
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
