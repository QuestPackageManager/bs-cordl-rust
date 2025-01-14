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
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::LocalVariable,
            >,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::QuoteInstruction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "QuoteInstruction";
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
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LocalVariable,
                >,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
                >),
                i32,
                1usize,
            >("Run")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Run", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (frame)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        operand: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        hoistedVariables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LocalVariable,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Linq::Expressions::ParameterExpression,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Linq::Expressions::Interpreter::LocalVariable,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (operand, hoistedVariables))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_InstructionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_InstructionName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_InstructionName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_ProducedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_ProducedStack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_ProducedStack", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
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
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::LocalVariable,
            >,
        >,
    >,
    pub _frame: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    >,
    pub _shadowedVars: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::HashSet_1<
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::ParameterExpression,
                    >,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+QuoteInstruction+ExpressionQuoter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::QuoteInstruction_ExpressionQuoter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "QuoteInstruction/ExpressionQuoter";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::IStrongBox,
                >,
                1usize,
            >("GetBox")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBox", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IStrongBox,
        > = unsafe { method.invoke_unchecked(self, (variable)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        hoistedVariables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LocalVariable,
                >,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::BlockExpression,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                1usize,
            >("VisitBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitBlock", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn VisitCatchBlock(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::CatchBlock,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
                1usize,
            >("VisitCatchBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitCatchBlock", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::CatchBlock,
        > = unsafe { method.invoke_unchecked(self, (node)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression_1<T>,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                1usize,
            >("VisitLambda")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitLambda", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = unsafe { method.invoke_unchecked(self, (node)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                1usize,
            >("VisitParameter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitParameter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        hoistedVariables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LocalVariable,
                >,
            >,
        >,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Linq::Expressions::ParameterExpression,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Linq::Expressions::Interpreter::LocalVariable,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (hoistedVariables, frame))
        };
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
