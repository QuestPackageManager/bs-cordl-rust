#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Dynamic::Utils::ExpressionUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Dynamic.Utils";
    const CLASS_NAME: &'static str = "ExpressionUtils";
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
#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
impl std::ops::Deref for crate::System::Dynamic::Utils::ExpressionUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
impl std::ops::DerefMut for crate::System::Dynamic::Utils::ExpressionUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
impl crate::System::Dynamic::Utils::ExpressionUtils {
    pub fn GetParametersForValidation(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        nodeKind: crate::System::Linq::Expressions::ExpressionType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParametersForValidation", (method, nodeKind))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequiresCanRead_Expression_Il2CppString0(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequiresCanRead", (expression, paramName))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequiresCanRead_i32_1(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequiresCanRead", (expression, paramName, idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReturnObject<T>(
        collectionOrT: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReturnObject", (collectionOrT))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReturnReadOnly<T>(
        collection: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IReadOnlyList_1<T>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReturnReadOnly", (collection))?;
        Ok(__cordl_ret.into())
    }
    pub fn SameElements<T>(
        replacement: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IEnumerable_1<T>,
            >,
        >,
        current: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SameElements", (replacement, current))?;
        Ok(__cordl_ret.into())
    }
    pub fn SameElementsInCollection<T>(
        replacement: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::ICollection_1<T>,
        >,
        current: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SameElementsInCollection", (replacement, current))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryQuote(
        parameterType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        argument: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryQuote", (parameterType, argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateArgumentCount_LambdaExpression1(
        lambda: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateArgumentCount", (lambda))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateArgumentCount_MethodBase_ExpressionType_i32_Il2CppArray0(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        nodeKind: crate::System::Linq::Expressions::ExpressionType,
        count: i32,
        pis: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateArgumentCount", (method, nodeKind, count, pis))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateArgumentTypes(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        nodeKind: crate::System::Linq::Expressions::ExpressionType,
        arguments: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                >,
            >,
        >,
        methodParamName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateArgumentTypes",
                (method, nodeKind, arguments, methodParamName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateOneArgument(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        nodeKind: crate::System::Linq::Expressions::ExpressionType,
        arguments: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        pi: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        methodParamName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        argumentParamName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateOneArgument",
                (
                    method,
                    nodeKind,
                    arguments,
                    pi,
                    methodParamName,
                    argumentParamName,
                    index,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::Utils::ExpressionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
