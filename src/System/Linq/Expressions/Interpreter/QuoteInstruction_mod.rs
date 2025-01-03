#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct QuoteInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _operand: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Expression,
    >,
    pub _hoistedVariables: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Linq::Expressions::ParameterExpression,
            *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::QuoteInstruction =>
    "System.Linq.Expressions.Interpreter"."QuoteInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::QuoteInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::QuoteInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction")]
impl crate::System::Linq::Expressions::Interpreter::QuoteInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+QuoteInstruction+ExpressionQuoter"
    )]
    pub type ExpressionQuoter = crate::System::Linq::Expressions::Interpreter::QuoteInstruction_ExpressionQuoter;
    pub fn New(
        operand: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        hoistedVariables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::System::Linq::Expressions::ParameterExpression,
                *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (operand, hoistedVariables))?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Run", (frame))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        operand: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        hoistedVariables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::System::Linq::Expressions::ParameterExpression,
                *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (operand, hoistedVariables))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstructionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_InstructionName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProducedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProducedStack", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::QuoteInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction+ExpressionQuoter")]
#[repr(C)]
#[derive(Debug)]
pub struct QuoteInstruction_ExpressionQuoter {
    __cordl_parent: crate::System::Linq::Expressions::ExpressionVisitor,
    pub _variables: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Linq::Expressions::ParameterExpression,
            *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
    >,
    pub _frame: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    >,
    pub _shadowedVars: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            *mut crate::System::Collections::Generic::HashSet_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction+ExpressionQuoter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::QuoteInstruction_ExpressionQuoter =>
    "System.Linq.Expressions.Interpreter"."QuoteInstruction/ExpressionQuoter"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction+ExpressionQuoter")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::QuoteInstruction_ExpressionQuoter {
    type Target = crate::System::Linq::Expressions::ExpressionVisitor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction+ExpressionQuoter")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::QuoteInstruction_ExpressionQuoter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction+ExpressionQuoter")]
impl crate::System::Linq::Expressions::Interpreter::QuoteInstruction_ExpressionQuoter {
    pub fn GetBox(
        &mut self,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::CompilerServices::IStrongBox>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IStrongBox,
        > = __cordl_object.invoke("GetBox", (variable))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        hoistedVariables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::System::Linq::Expressions::ParameterExpression,
                *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
            >,
        >,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hoistedVariables, frame))?;
        Ok(__cordl_object.into())
    }
    pub fn VisitBlock(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("VisitBlock", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn VisitCatchBlock(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::CatchBlock,
        > = __cordl_object.invoke("VisitCatchBlock", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn VisitLambda<T>(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("VisitLambda", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn VisitParameter(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("VisitParameter", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        hoistedVariables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::System::Linq::Expressions::ParameterExpression,
                *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
            >,
        >,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hoistedVariables, frame))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction+ExpressionQuoter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::QuoteInstruction_ExpressionQuoter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
