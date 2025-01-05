#[cfg(feature = "System+Linq+Expressions+Expression")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::Expression =>
    "System.Linq.Expressions"."Expression"
);
#[cfg(feature = "System+Linq+Expressions+Expression")]
impl std::ops::Deref for crate::System::Linq::Expressions::Expression {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Expression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression")]
impl crate::System::Linq::Expressions::Expression {
    #[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
    pub type BinaryExpressionProxy = crate::System::Linq::Expressions::Expression_BinaryExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
    pub type BlockExpressionProxy = crate::System::Linq::Expressions::Expression_BlockExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
    pub type CatchBlockProxy = crate::System::Linq::Expressions::Expression_CatchBlockProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
    pub type ConditionalExpressionProxy = crate::System::Linq::Expressions::Expression_ConditionalExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
    pub type ConstantExpressionProxy = crate::System::Linq::Expressions::Expression_ConstantExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
    pub type DebugInfoExpressionProxy = crate::System::Linq::Expressions::Expression_DebugInfoExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
    pub type DefaultExpressionProxy = crate::System::Linq::Expressions::Expression_DefaultExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
    pub type ExtensionInfo = crate::System::Linq::Expressions::Expression_ExtensionInfo;
    #[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
    pub type GotoExpressionProxy = crate::System::Linq::Expressions::Expression_GotoExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
    pub type IndexExpressionProxy = crate::System::Linq::Expressions::Expression_IndexExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
    pub type InvocationExpressionProxy = crate::System::Linq::Expressions::Expression_InvocationExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
    pub type LabelExpressionProxy = crate::System::Linq::Expressions::Expression_LabelExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
    pub type LambdaExpressionProxy = crate::System::Linq::Expressions::Expression_LambdaExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
    pub type ListInitExpressionProxy = crate::System::Linq::Expressions::Expression_ListInitExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
    pub type LoopExpressionProxy = crate::System::Linq::Expressions::Expression_LoopExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
    pub type MemberExpressionProxy = crate::System::Linq::Expressions::Expression_MemberExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
    pub type MemberInitExpressionProxy = crate::System::Linq::Expressions::Expression_MemberInitExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
    pub type MethodCallExpressionProxy = crate::System::Linq::Expressions::Expression_MethodCallExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
    pub type NewArrayExpressionProxy = crate::System::Linq::Expressions::Expression_NewArrayExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
    pub type NewExpressionProxy = crate::System::Linq::Expressions::Expression_NewExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
    pub type ParameterExpressionProxy = crate::System::Linq::Expressions::Expression_ParameterExpressionProxy;
    #[cfg(
        feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy"
    )]
    pub type RuntimeVariablesExpressionProxy = crate::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
    pub type SwitchCaseProxy = crate::System::Linq::Expressions::Expression_SwitchCaseProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
    pub type SwitchExpressionProxy = crate::System::Linq::Expressions::Expression_SwitchExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
    pub type TryExpressionProxy = crate::System::Linq::Expressions::Expression_TryExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
    pub type TypeBinaryExpressionProxy = crate::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
    pub type UnaryExpressionProxy = crate::System::Linq::Expressions::Expression_UnaryExpressionProxy;
    pub fn Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ExpressionVisitor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("Accept", (visitor))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddAssignChecked(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddAssignChecked", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddChecked(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddChecked", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn And(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("And", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn AndAlso_Gc1(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AndAlso", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn AndAlso_Gc_Gc0(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AndAlso", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn AndAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AndAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyTypeArgs(
        m: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        typeArgs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyTypeArgs", (m, typeArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArrayAccess_Gc_Gc0(
        array: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        indexes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::IndexExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::IndexExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArrayAccess", (array, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArrayAccess_Gc_Gc1(
        array: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        indexes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::IndexExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::IndexExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArrayAccess", (array, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArrayIndex(
        array: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        index: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArrayIndex", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArrayLength(
        array: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArrayLength", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn Assign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Assign", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn BlockCore(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        variables: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
        expressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BlockCore", (_cordl_type, variables, expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc0(
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (arg0, arg1))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc4(
        expressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc5(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        expressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (_cordl_type, expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc6(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        expressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (_cordl_type, expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc7(
        variables: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
        expressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (variables, expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc9(
        variables: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
        expressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (variables, expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc_Gc1(
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (arg0, arg1, arg2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc_Gc10(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        variables: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
        expressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (_cordl_type, variables, expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc_Gc8(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        variables: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
        expressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (_cordl_type, variables, expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc_Gc_Gc2(
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (arg0, arg1, arg2, arg3))?;
        Ok(__cordl_ret.into())
    }
    pub fn Block_Gc_Gc_Gc_Gc3(
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg4: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (arg0, arg1, arg2, arg3, arg4))?;
        Ok(__cordl_ret.into())
    }
    pub fn Break(
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::GotoExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::GotoExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Break", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc0(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Call", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc1(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (method, arg0))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc6(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (method, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc7(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (method, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc8(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (instance, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc_Gc10(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (instance, method, arg0))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc_Gc14(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (instance, method, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc_Gc2(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (method, arg0, arg1))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc_Gc9(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (instance, method, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc_Gc_Gc11(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (instance, method, arg0, arg1))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc_Gc_Gc13(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeArguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (instance, methodName, typeArguments, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc_Gc_Gc3(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (method, arg0, arg1, arg2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc_Gc_Gc_Gc12(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (instance, method, arg0, arg1, arg2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc_Gc_Gc_Gc4(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (method, arg0, arg1, arg2, arg3))?;
        Ok(__cordl_ret.into())
    }
    pub fn Call_Gc_Gc_Gc_Gc_Gc5(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg4: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (method, arg0, arg1, arg2, arg3, arg4))?;
        Ok(__cordl_ret.into())
    }
    pub fn Coalesce(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Coalesce", (left, right, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn Condition_Gc1(
        test: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        ifTrue: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        ifFalse: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConditionalExpression,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConditionalExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Condition", (test, ifTrue, ifFalse, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Condition_Gc_Gc_Gc0(
        test: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        ifTrue: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        ifFalse: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConditionalExpression,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConditionalExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Condition", (test, ifTrue, ifFalse))?;
        Ok(__cordl_ret.into())
    }
    pub fn Constant_Gc0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ConstantExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConstantExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Constant", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Constant_Gc1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ConstantExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConstantExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Constant", (value, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertChecked(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertChecked", (expression, _cordl_type, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Convert_Gc1(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Convert", (expression, _cordl_type, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Convert_Gc_Gc0(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Convert", (expression, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLambda(
        delegateType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tailCall: bool,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LambdaExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateLambda", (delegateType, body, name, tailCall, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decrement(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decrement", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Default(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::DefaultExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::DefaultExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Default", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Divide(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Divide", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn DivideAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DivideAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn Empty() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::DefaultExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::DefaultExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Empty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equal_Gc_Gc0(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Equal", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equal__cordl_bool_Gc1(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Equal", (left, right, liftToNull, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExclusiveOr(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExclusiveOr", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExclusiveOrAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExclusiveOrAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn Field_Gc_Gc0(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MemberExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MemberExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Field", (expression, field))?;
        Ok(__cordl_ret.into())
    }
    pub fn Field_Gc_Gc1(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MemberExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MemberExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Field", (expression, fieldName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMethod(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeArgs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
        flags: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindMethod", (_cordl_type, methodName, typeArgs, args, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComparisonOperator(
        binaryType: crate::System::Linq::Expressions::ExpressionType,
        opName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetComparisonOperator",
                (binaryType, opName, left, right, liftToNull),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEqualityComparisonOperator(
        binaryType: crate::System::Linq::Expressions::ExpressionType,
        opName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetEqualityComparisonOperator",
                (binaryType, opName, left, right, liftToNull),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInvokeMethod(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInvokeMethod", (expression))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodBasedAssignOperator(
        binaryType: crate::System::Linq::Expressions::ExpressionType,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
        liftToNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetMethodBasedAssignOperator",
                (binaryType, left, right, method, conversion, liftToNull),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodBasedBinaryOperator(
        binaryType: crate::System::Linq::Expressions::ExpressionType,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        liftToNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetMethodBasedBinaryOperator",
                (binaryType, left, right, method, liftToNull),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodBasedCoercionOperator(
        unaryType: crate::System::Linq::Expressions::ExpressionType,
        operand: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        convertToType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetMethodBasedCoercionOperator",
                (unaryType, operand, convertToType, method),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodBasedUnaryOperator(
        unaryType: crate::System::Linq::Expressions::ExpressionType,
        operand: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodBasedUnaryOperator", (unaryType, operand, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOptimizedBlockExpression(
        expressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOptimizedBlockExpression", (expressions))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn GetResultTypeOfShift(
        left: quest_hook::libil2cpp::Gc<crate::System::Type>,
        right: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResultTypeOfShift", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDefinedAssignOperatorOrThrow(
        binaryType: crate::System::Linq::Expressions::ExpressionType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
        liftToNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetUserDefinedAssignOperatorOrThrow",
                (binaryType, name, left, right, conversion, liftToNull),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDefinedBinaryOperatorOrThrow(
        binaryType: crate::System::Linq::Expressions::ExpressionType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetUserDefinedBinaryOperatorOrThrow",
                (binaryType, name, left, right, liftToNull),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDefinedBinaryOperator_ExpressionType_Gc_Gc_Gc1(
        binaryType: crate::System::Linq::Expressions::ExpressionType,
        leftType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        rightType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetUserDefinedBinaryOperator",
                (binaryType, leftType, rightType, name),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDefinedBinaryOperator__cordl_bool0(
        binaryType: crate::System::Linq::Expressions::ExpressionType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetUserDefinedBinaryOperator",
                (binaryType, name, left, right, liftToNull),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDefinedCoercion(
        coercionType: crate::System::Linq::Expressions::ExpressionType,
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        convertToType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetUserDefinedCoercion",
                (coercionType, expression, convertToType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDefinedCoercionOrThrow(
        coercionType: crate::System::Linq::Expressions::ExpressionType,
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        convertToType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetUserDefinedCoercionOrThrow",
                (coercionType, expression, convertToType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDefinedUnaryOperator(
        unaryType: crate::System::Linq::Expressions::ExpressionType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        operand: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUserDefinedUnaryOperator", (unaryType, name, operand))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDefinedUnaryOperatorOrThrow(
        unaryType: crate::System::Linq::Expressions::ExpressionType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        operand: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUserDefinedUnaryOperatorOrThrow", (unaryType, name, operand))?;
        Ok(__cordl_ret.into())
    }
    pub fn Goto_Gc_Gc0(
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::GotoExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::GotoExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Goto", (target, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Goto_Gc_Gc1(
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        value: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::GotoExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::GotoExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Goto", (target, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GreaterThan(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GreaterThan", (left, right, liftToNull, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn GreaterThanOrEqual(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GreaterThanOrEqual", (left, right, liftToNull, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn IfThen(
        test: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        ifTrue: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConditionalExpression,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConditionalExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IfThen", (test, ifTrue))?;
        Ok(__cordl_ret.into())
    }
    pub fn Increment(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Increment", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke_Gc0(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::InvocationExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::InvocationExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Invoke", (expression))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke_Gc1(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::InvocationExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::InvocationExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Invoke", (expression, arg0))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke_Gc6(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::InvocationExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::InvocationExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Invoke", (expression, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke_Gc_Gc2(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::InvocationExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::InvocationExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Invoke", (expression, arg0, arg1))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke_Gc_Gc_Gc3(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::InvocationExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::InvocationExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Invoke", (expression, arg0, arg1, arg2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke_Gc_Gc_Gc_Gc4(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::InvocationExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::InvocationExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Invoke", (expression, arg0, arg1, arg2, arg3))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke_Gc_Gc_Gc_Gc_Gc5(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg4: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::InvocationExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::InvocationExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Invoke", (expression, arg0, arg1, arg2, arg3, arg4))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCompatible(
        m: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCompatible", (m, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFalse(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsFalse", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLiftingConditionalLogicalOperator(
        left: quest_hook::libil2cpp::Gc<crate::System::Type>,
        right: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        binaryType: crate::System::Linq::Expressions::ExpressionType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "IsLiftingConditionalLogicalOperator",
                (left, right, method, binaryType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNullComparison(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNullComparison", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNullConstant(
        e: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNullConstant", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSimpleShift(
        left: quest_hook::libil2cpp::Gc<crate::System::Type>,
        right: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSimpleShift", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTrue(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTrue", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidLiftedConditionalLogicalOperator(
        left: quest_hook::libil2cpp::Gc<crate::System::Type>,
        right: quest_hook::libil2cpp::Gc<crate::System::Type>,
        pms: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidLiftedConditionalLogicalOperator", (left, right, pms))?;
        Ok(__cordl_ret.into())
    }
    pub fn Label_2() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelTarget,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Label", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Label_Gc0(
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Label", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn Label_Gc3(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelTarget,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Label", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn Label_Gc4(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelTarget,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Label", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Label_Gc_Gc1(
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        defaultValue: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Label", (target, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn Label_Gc_Gc5(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelTarget,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Label", (_cordl_type, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lambda_Gc0<TDelegate>(
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TDelegate>>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TDelegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lambda", (body, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lambda_Gc1<TDelegate>(
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TDelegate>>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TDelegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lambda", (body, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lambda_Gc_Gc4(
        delegateType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LambdaExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lambda", (delegateType, body, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lambda_Gc_Gc__cordl_bool_Gc5(
        delegateType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tailCall: bool,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LambdaExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lambda", (delegateType, body, name, tailCall, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lambda_Gc__cordl_bool_Gc3<TDelegate>(
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tailCall: bool,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TDelegate>>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TDelegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lambda", (body, name, tailCall, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lambda__cordl_bool_Gc2<TDelegate>(
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        tailCall: bool,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TDelegate>>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TDelegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lambda", (body, tailCall, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn LeftShift(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LeftShift", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn LeftShiftAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LeftShiftAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn LessThan(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LessThan", (left, right, liftToNull, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn LessThanOrEqual(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LessThanOrEqual", (left, right, liftToNull, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Loop(
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        _cordl_break: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelTarget,
        >,
        _cordl_continue: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelTarget,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LoopExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LoopExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Loop", (body, _cordl_break, _cordl_continue))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeBinary_ExpressionType_Gc_Gc__cordl_bool_Gc0(
        binaryType: crate::System::Linq::Expressions::ExpressionType,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeBinary", (binaryType, left, right, liftToNull, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeBinary_Gc1(
        binaryType: crate::System::Linq::Expressions::ExpressionType,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MakeBinary",
                (binaryType, left, right, liftToNull, method, conversion),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeCatchBlock(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        filter: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::CatchBlock,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeCatchBlock", (_cordl_type, variable, body, filter))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeGoto(
        kind: crate::System::Linq::Expressions::GotoExpressionKind,
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        value: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::GotoExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::GotoExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeGoto", (kind, target, value, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeIndex(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        indexer: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::IndexExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::IndexExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeIndex", (instance, indexer, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeIndexProperty(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        indexer: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        argList: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::IndexExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::IndexExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeIndexProperty", (instance, indexer, paramName, argList))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeMemberAccess(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MemberExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MemberExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeMemberAccess", (expression, member))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeOpAssignUnary(
        kind: crate::System::Linq::Expressions::ExpressionType,
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeOpAssignUnary", (kind, expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeTry(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        finally: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        fault: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        handlers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::TryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::TryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeTry", (_cordl_type, body, finally, fault, handlers))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeUnary(
        unaryType: crate::System::Linq::Expressions::ExpressionType,
        operand: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeUnary", (unaryType, operand, _cordl_type, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Modulo(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Modulo", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn ModuloAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ModuloAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn Multiply(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Multiply", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyAssignChecked(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyAssignChecked", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyChecked(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyChecked", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Negate(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Negate", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn NegateChecked(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NegateChecked", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NewArrayBounds(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bounds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::NewArrayExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::NewArrayExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewArrayBounds", (_cordl_type, bounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewArrayInit_Gc_Gc0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        initializers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::NewArrayExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::NewArrayExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewArrayInit", (_cordl_type, initializers))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewArrayInit_Gc_Gc1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        initializers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::NewArrayExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::NewArrayExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewArrayInit", (_cordl_type, initializers))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotEqual_Gc_Gc0(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotEqual", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotEqual__cordl_bool_Gc1(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftToNull: bool,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotEqual", (left, right, liftToNull, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Not_Gc0(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Not", (expression))?;
        Ok(__cordl_ret.into())
    }
    pub fn Not_Gc1(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Not", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnesComplement(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnesComplement", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Or(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Or", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrElse(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrElse", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParameterIsAssignable(
        pi: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        argType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParameterIsAssignable", (pi, argType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parameter_Gc0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ParameterExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parameter", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parameter_Gc1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ParameterExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parameter", (_cordl_type, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn PostDecrementAssign(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PostDecrementAssign", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn PostIncrementAssign(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PostIncrementAssign", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Power(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Power", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn PowerAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PowerAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreDecrementAssign(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreDecrementAssign", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreIncrementAssign_Gc0(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreIncrementAssign", (expression))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreIncrementAssign_Gc1(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreIncrementAssign", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Property_Gc0(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        indexer: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::IndexExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::IndexExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Property", (instance, indexer, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn Property_Gc_Gc1(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MemberExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MemberExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Property", (expression, propertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Property_Gc_Gc2(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MemberExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MemberExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Property", (expression, property))?;
        Ok(__cordl_ret.into())
    }
    pub fn Quote(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Quote", (expression))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reduce(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("Reduce", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReduceAndCheck(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("ReduceAndCheck", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReferenceEqual(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReferenceEqual", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReferenceNotEqual(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReferenceNotEqual", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequiresCanRead(
        items: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequiresCanRead", (items, paramName))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequiresCanWrite(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequiresCanWrite", (expression, paramName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Return_Gc0(
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::GotoExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::GotoExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Return", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn Return_Gc1(
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        value: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::GotoExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::GotoExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Return", (target, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn RightShift(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RightShift", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn RightShiftAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RightShiftAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Subtract", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubtractAssign(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubtractAssign", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubtractAssignChecked(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubtractAssignChecked", (left, right, method, conversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubtractChecked(
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubtractChecked", (left, right, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Throw(
        value: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Throw", (value, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFinally(
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        finally: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::TryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::TryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFinally", (body, finally))?;
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
    pub fn TypeAs(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TypeAs", (expression, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn TypeEqual(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::TypeBinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::TypeBinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TypeEqual", (expression, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn TypeIs(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::TypeBinaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::TypeBinaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TypeIs", (expression, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnaryPlus(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnaryPlus", (expression, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbox(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unbox", (expression, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Validate(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        allowByRef: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Validate", (_cordl_type, allowByRef))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateAccessor(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        indexes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
            >,
        >,
        arguments: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateAccessor",
                (instance, method, indexes, arguments, paramName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateAccessorArgumentTypes(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        indexes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
            >,
        >,
        arguments: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateAccessorArgumentTypes",
                (method, indexes, arguments, paramName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateArgumentCount(
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
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
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
    pub fn ValidateCallInstanceType(
        instanceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateCallInstanceType", (instanceType, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateCoalesceArgTypes(
        left: quest_hook::libil2cpp::Gc<crate::System::Type>,
        right: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateCoalesceArgTypes", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateGoto(
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
        targetParameter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valueParameter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateGoto",
                (target, value, targetParameter, valueParameter, _cordl_type),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateGotoType(
        expectedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateGotoType", (expectedType, value, paramName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateIndexedProperty(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        indexer: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        argList: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateIndexedProperty", (instance, indexer, paramName, argList))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateLambdaArgs(
        delegateType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        body: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateLambdaArgs", (delegateType, body, parameters, paramName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateMethodAndGetParameters(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
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
            .invoke("ValidateMethodAndGetParameters", (instance, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateMethodInfo(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateMethodInfo", (method, paramName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateOneArgument(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        nodeKind: crate::System::Linq::Expressions::ExpressionType,
        arg: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        pi: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        methodParamName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        argumentParamName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateOneArgument",
                (method, nodeKind, arg, pi, methodParamName, argumentParamName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateOpAssignConversionLambda(
        conversion: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        nodeType: crate::System::Linq::Expressions::ExpressionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateOpAssignConversionLambda",
                (conversion, left, method, nodeType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateOperator(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateOperator", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateParamswithOperandsOrThrow(
        paramType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        operandType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        exprType: crate::System::Linq::Expressions::ExpressionType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateParamswithOperandsOrThrow",
                (paramType, operandType, exprType, name),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateStaticOrInstanceMethod(
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateStaticOrInstanceMethod", (instance, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateTryAndCatchHaveSameType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        tryBody: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        handlers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateTryAndCatchHaveSameType",
                (_cordl_type, tryBody, handlers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateUserDefinedConditionalLogicOperator(
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        left: quest_hook::libil2cpp::Gc<crate::System::Type>,
        right: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateUserDefinedConditionalLogicOperator",
                (nodeType, left, right, method),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateVariables(
        varList: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
        collectionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateVariables", (varList, collectionName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Variable(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ParameterExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Variable", (_cordl_type, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyOpTrueFalse(
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        left: quest_hook::libil2cpp::Gc<crate::System::Type>,
        opTrue: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyOpTrueFalse", (nodeType, left, opTrue, paramName))?;
        Ok(__cordl_ret.into())
    }
    pub fn VisitChildren(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ExpressionVisitor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("VisitChildren", (visitor))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanReduce(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanReduce", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::ExpressionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::ExpressionType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Expressions::Expression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_BinaryExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_BinaryExpressionProxy =>
    "System.Linq.Expressions"."Expression/BinaryExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_BinaryExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_BinaryExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_BinaryExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_BinaryExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_BlockExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_BlockExpressionProxy =>
    "System.Linq.Expressions"."Expression/BlockExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_BlockExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_BlockExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_BlockExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_BlockExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_CatchBlockProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_CatchBlockProxy => "System.Linq.Expressions"
    ."Expression/CatchBlockProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
impl std::ops::Deref for crate::System::Linq::Expressions::Expression_CatchBlockProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_CatchBlockProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
impl crate::System::Linq::Expressions::Expression_CatchBlockProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_CatchBlockProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_ConditionalExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_ConditionalExpressionProxy =>
    "System.Linq.Expressions"."Expression/ConditionalExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_ConditionalExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_ConditionalExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_ConditionalExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_ConditionalExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_ConstantExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_ConstantExpressionProxy =>
    "System.Linq.Expressions"."Expression/ConstantExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_ConstantExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_ConstantExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_ConstantExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_ConstantExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_DebugInfoExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_DebugInfoExpressionProxy =>
    "System.Linq.Expressions"."Expression/DebugInfoExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_DebugInfoExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_DebugInfoExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_DebugInfoExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_DebugInfoExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_DefaultExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_DefaultExpressionProxy =>
    "System.Linq.Expressions"."Expression/DefaultExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_DefaultExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_DefaultExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_DefaultExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_DefaultExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_ExtensionInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub NodeType: crate::System::Linq::Expressions::ExpressionType,
    pub Type: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_ExtensionInfo => "System.Linq.Expressions"
    ."Expression/ExtensionInfo"
);
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
impl std::ops::Deref for crate::System::Linq::Expressions::Expression_ExtensionInfo {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Expression_ExtensionInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
impl crate::System::Linq::Expressions::Expression_ExtensionInfo {}
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_ExtensionInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_GotoExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_GotoExpressionProxy =>
    "System.Linq.Expressions"."Expression/GotoExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_GotoExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_GotoExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_GotoExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_GotoExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_IndexExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_IndexExpressionProxy =>
    "System.Linq.Expressions"."Expression/IndexExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_IndexExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_IndexExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_IndexExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_IndexExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_InvocationExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_InvocationExpressionProxy =>
    "System.Linq.Expressions"."Expression/InvocationExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_InvocationExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_InvocationExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_InvocationExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_InvocationExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_LabelExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_LabelExpressionProxy =>
    "System.Linq.Expressions"."Expression/LabelExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_LabelExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_LabelExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_LabelExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_LabelExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_LambdaExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_LambdaExpressionProxy =>
    "System.Linq.Expressions"."Expression/LambdaExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_LambdaExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_LambdaExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_LambdaExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_LambdaExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_ListInitExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_ListInitExpressionProxy =>
    "System.Linq.Expressions"."Expression/ListInitExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_ListInitExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_ListInitExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_ListInitExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_ListInitExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_LoopExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_LoopExpressionProxy =>
    "System.Linq.Expressions"."Expression/LoopExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_LoopExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_LoopExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_LoopExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_LoopExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_MemberExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_MemberExpressionProxy =>
    "System.Linq.Expressions"."Expression/MemberExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_MemberExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_MemberExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_MemberExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_MemberExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_MemberInitExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_MemberInitExpressionProxy =>
    "System.Linq.Expressions"."Expression/MemberInitExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_MemberInitExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_MemberInitExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_MemberInitExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_MemberInitExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_MethodCallExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_MethodCallExpressionProxy =>
    "System.Linq.Expressions"."Expression/MethodCallExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_MethodCallExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_MethodCallExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_MethodCallExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_MethodCallExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_NewArrayExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_NewArrayExpressionProxy =>
    "System.Linq.Expressions"."Expression/NewArrayExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_NewArrayExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_NewArrayExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_NewArrayExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_NewArrayExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_NewExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_NewExpressionProxy =>
    "System.Linq.Expressions"."Expression/NewExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_NewExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_NewExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_NewExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_NewExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_ParameterExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_ParameterExpressionProxy =>
    "System.Linq.Expressions"."Expression/ParameterExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_ParameterExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_ParameterExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_ParameterExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_ParameterExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_RuntimeVariablesExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy =>
    "System.Linq.Expressions"."Expression/RuntimeVariablesExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_SwitchCaseProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_SwitchCaseProxy => "System.Linq.Expressions"
    ."Expression/SwitchCaseProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
impl std::ops::Deref for crate::System::Linq::Expressions::Expression_SwitchCaseProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_SwitchCaseProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
impl crate::System::Linq::Expressions::Expression_SwitchCaseProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_SwitchCaseProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_SwitchExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_SwitchExpressionProxy =>
    "System.Linq.Expressions"."Expression/SwitchExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_SwitchExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_SwitchExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_SwitchExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_SwitchExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_TryExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_TryExpressionProxy =>
    "System.Linq.Expressions"."Expression/TryExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_TryExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_TryExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_TryExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_TryExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_TypeBinaryExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy =>
    "System.Linq.Expressions"."Expression/TypeBinaryExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_UnaryExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_UnaryExpressionProxy =>
    "System.Linq.Expressions"."Expression/UnaryExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_UnaryExpressionProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_UnaryExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_UnaryExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_UnaryExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
