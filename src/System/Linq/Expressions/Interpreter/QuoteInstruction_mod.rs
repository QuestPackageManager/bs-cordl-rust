#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct QuoteInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _operand: *mut crate::System::Linq::Expressions::Expression,
    pub _hoistedVariables: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Linq::Expressions::ParameterExpression,
        *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
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
        operand: *mut crate::System::Linq::Expressions::Expression,
        hoistedVariables: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Linq::Expressions::ParameterExpression,
            *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (operand, hoistedVariables))?;
        Ok(__cordl_object)
    }
    pub fn Run(
        &mut self,
        frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Run", (frame))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        operand: *mut crate::System::Linq::Expressions::Expression,
        hoistedVariables: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Linq::Expressions::ParameterExpression,
            *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (operand, hoistedVariables))?;
        Ok(__cordl_ret)
    }
    pub fn get_InstructionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_InstructionName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProducedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProducedStack", ())?;
        Ok(__cordl_ret)
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
    pub _variables: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Linq::Expressions::ParameterExpression,
        *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
    >,
    pub _frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    pub _shadowedVars: *mut crate::System::Collections::Generic::Stack_1<
        *mut crate::System::Collections::Generic::HashSet_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
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
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::CompilerServices::IStrongBox,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::CompilerServices::IStrongBox = __cordl_object
            .invoke("GetBox", (variable))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        hoistedVariables: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Linq::Expressions::ParameterExpression,
            *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
        frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hoistedVariables, frame))?;
        Ok(__cordl_object)
    }
    pub fn VisitBlock(
        &mut self,
        node: *mut crate::System::Linq::Expressions::BlockExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitBlock", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitCatchBlock(
        &mut self,
        node: *mut crate::System::Linq::Expressions::CatchBlock,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::CatchBlock,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::CatchBlock = __cordl_object
            .invoke("VisitCatchBlock", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitLambda<T>(
        &mut self,
        node: *mut crate::System::Linq::Expressions::Expression_1<T>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Linq::Expressions::Expression>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitLambda", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitParameter(
        &mut self,
        node: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitParameter", (node))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        hoistedVariables: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Linq::Expressions::ParameterExpression,
            *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
        frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hoistedVariables, frame))?;
        Ok(__cordl_ret)
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
